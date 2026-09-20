//! The macOS OCR engine (Vision's text recognition), reached through raw Objective-C messaging.
//!
//! Vision ships its recognition languages with the OS, so unlike Windows there is no language
//! pack to install and no `NoLanguage` case to report in practice. The framework calls are
//! declared here rather than pulled in as crates: this is the only place in the app that needs
//! CoreGraphics, and one small extern block is less to carry than three more dependencies.

use std::{
	ffi::{CStr, c_void},
	ptr,
};

use objc::{class, msg_send, rc::autoreleasepool, runtime::Object, sel, sel_impl};

use super::OcrError;

type CFTypeRef = *const c_void;
type CFDataRef = *const c_void;
type CGColorSpaceRef = *mut c_void;
type CGDataProviderRef = *mut c_void;
type CGImageRef = *mut c_void;

/// RGBA8, alpha last and not premultiplied, which is what pdfium hands us.
const K_CG_IMAGE_ALPHA_LAST: u32 = 3;
const K_CG_RENDERING_INTENT_DEFAULT: i32 = 0;
/// `VNRequestTextRecognitionLevelAccurate`. Slower than the fast level and worth it: the fast
/// level is built for short strings in camera frames, not pages of body text.
const RECOGNITION_LEVEL_ACCURATE: isize = 0;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
	fn CFDataCreate(allocator: CFTypeRef, bytes: *const u8, length: isize) -> CFDataRef;
	fn CFRelease(cf: CFTypeRef);
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
	fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
	fn CGColorSpaceRelease(space: CGColorSpaceRef);
	fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
	fn CGDataProviderRelease(provider: CGDataProviderRef);
	#[allow(clippy::too_many_arguments)]
	fn CGImageCreate(
		width: usize,
		height: usize,
		bits_per_component: usize,
		bits_per_pixel: usize,
		bytes_per_row: usize,
		space: CGColorSpaceRef,
		bitmap_info: u32,
		provider: CGDataProviderRef,
		decode: *const f64,
		should_interpolate: bool,
		intent: i32,
	) -> CGImageRef;
	fn CGImageRelease(image: CGImageRef);
}

// Linked for the Vision classes looked up by name below; nothing here calls into it directly.
#[link(name = "Vision", kind = "framework")]
unsafe extern "C" {}

/// Vision has no documented input cap the way `Windows.Media.Ocr` does. This keeps a rendered
/// page around the size of a 300 DPI letter scan, which is what the recognizer wants anyway and
/// keeps one page's pixels near 40 MB rather than several times that.
pub fn max_image_dimension() -> u32 {
	4000
}

pub fn recognize_rgba(rgba: &[u8], width: u32, height: u32) -> Result<String, OcrError> {
	let image = CgImage::new(rgba, width, height)?;
	autoreleasepool(|| unsafe { recognize_image(image.0) })
}

/// A `CGImage` and the CoreGraphics objects behind it, released together.
struct CgImage(CGImageRef);

impl CgImage {
	fn new(rgba: &[u8], width: u32, height: u32) -> Result<Self, OcrError> {
		let length = isize::try_from(rgba.len()).map_err(|_| OcrError::Failed("bitmap too large".to_string()))?;
		// CFDataCreate copies, so the image does not borrow from `rgba`.
		let data = unsafe { CFDataCreate(ptr::null(), rgba.as_ptr(), length) };
		if data.is_null() {
			return Err(OcrError::Failed("allocating the bitmap failed".to_string()));
		}
		let provider = unsafe { CGDataProviderCreateWithCFData(data) };
		let space = unsafe { CGColorSpaceCreateDeviceRGB() };
		let image = if provider.is_null() || space.is_null() {
			ptr::null_mut()
		} else {
			unsafe {
				CGImageCreate(
					width as usize,
					height as usize,
					8,
					32,
					width as usize * 4,
					space,
					K_CG_IMAGE_ALPHA_LAST,
					provider,
					ptr::null(),
					false,
					K_CG_RENDERING_INTENT_DEFAULT,
				)
			}
		};
		unsafe {
			if !space.is_null() {
				CGColorSpaceRelease(space);
			}
			if !provider.is_null() {
				CGDataProviderRelease(provider);
			}
			CFRelease(data);
		}
		if image.is_null() {
			return Err(OcrError::Failed("building the bitmap failed".to_string()));
		}
		Ok(Self(image))
	}
}

impl Drop for CgImage {
	fn drop(&mut self) {
		unsafe { CGImageRelease(self.0) };
	}
}

/// Runs one accurate text-recognition request over `image` and joins the recognized lines.
unsafe fn recognize_image(image: CGImageRef) -> Result<String, OcrError> {
	let request: *mut Object = msg_send![class!(VNRecognizeTextRequest), new];
	if request.is_null() {
		return Err(OcrError::Failed("Vision text recognition is unavailable".to_string()));
	}
	let _: () = msg_send![request, setRecognitionLevel: RECOGNITION_LEVEL_ACCURATE];
	let _: () = msg_send![request, setUsesLanguageCorrection: true];
	let options: *mut Object = msg_send![class!(NSDictionary), dictionary];
	let handler: *mut Object = msg_send![class!(VNImageRequestHandler), alloc];
	let handler: *mut Object = msg_send![handler, initWithCGImage: image options: options];
	if handler.is_null() {
		let _: () = msg_send![request, release];
		return Err(OcrError::Failed("preparing the page for Vision failed".to_string()));
	}
	let requests: *mut Object = msg_send![class!(NSArray), arrayWithObject: request];
	let mut error: *mut Object = ptr::null_mut();
	let performed: bool = msg_send![handler, performRequests: requests error: &mut error];
	let text = if performed { Ok(collect_text(request)) } else { Err(OcrError::Failed(error_message(error))) };
	let _: () = msg_send![handler, release];
	let _: () = msg_send![request, release];
	text
}

/// Joins the top candidate of every recognized observation, one per line, in the order Vision
/// reports them.
unsafe fn collect_text(request: *mut Object) -> String {
	let results: *mut Object = msg_send![request, results];
	if results.is_null() {
		return String::new();
	}
	let count: usize = msg_send![results, count];
	let mut lines = Vec::with_capacity(count);
	for index in 0..count {
		let observation: *mut Object = msg_send![results, objectAtIndex: index];
		let candidates: *mut Object = msg_send![observation, topCandidates: 1_usize];
		if candidates.is_null() {
			continue;
		}
		let candidate_count: usize = msg_send![candidates, count];
		if candidate_count == 0 {
			continue;
		}
		let candidate: *mut Object = msg_send![candidates, objectAtIndex: 0_usize];
		let string: *mut Object = msg_send![candidate, string];
		if let Some(line) = ns_string_to_rust(string) {
			lines.push(line);
		}
	}
	lines.join("\n")
}

unsafe fn error_message(error: *mut Object) -> String {
	if error.is_null() {
		return "recognition failed".to_string();
	}
	let description: *mut Object = msg_send![error, localizedDescription];
	ns_string_to_rust(description).unwrap_or_else(|| "recognition failed".to_string())
}

unsafe fn ns_string_to_rust(string: *mut Object) -> Option<String> {
	if string.is_null() {
		return None;
	}
	let utf8: *const std::ffi::c_char = msg_send![string, UTF8String];
	if utf8.is_null() {
		return None;
	}
	Some(CStr::from_ptr(utf8).to_string_lossy().into_owned())
}
