use patois::t;
use wxdragon::{event::WebViewEvents, prelude::*, widgets::WebView};

type NavigationHandler = Box<dyn Fn(&str) -> bool>;
use std::{cell::Cell, rc::Rc};

use wx_utils::dpi;

thread_local! {
	pub static ACTIVE_WEB_VIEW: Cell<Option<Dialog>> = const { Cell::new(None) };
}

pub fn show_web_view_dialog(
	parent: &Frame,
	title: &str,
	url_or_content: &str,
	is_url: bool,
	navigation_handler: Option<NavigationHandler>,
) {
	let dialog = Dialog::builder(parent, title)
		.with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
		.with_size(dpi::scale(parent, 800), dpi::scale(parent, 600))
		.build();
	ACTIVE_WEB_VIEW.with(|v| v.set(Some(dialog)));
	let web_view = WebView::builder(&dialog).build();
	web_view.add_script_message_handler("wx");
	// EndModal asserts if it runs on a dialog that is no longer modal, so every path
	// that closes this dialog goes through `closing` and only the first one acts. The
	// escape listener is installed on every load, and following a fragment link (the
	// kind a table of contents is full of) completes a load without replacing the
	// document, so a page could end up carrying several of them and a single Escape
	// could post several close messages.
	let closing = Rc::new(Cell::new(false));
	let dialog_for_close = dialog;
	let closing_for_script = Rc::clone(&closing);
	web_view.on_script_message_received(move |event| {
		if event.get_string() == Some("close_dialog".to_string()) && !closing_for_script.replace(true) {
			dialog_for_close.end_modal(ID_CANCEL);
		}
	});
	if let Some(handler) = navigation_handler {
		web_view.on_navigating(move |event| {
			if let Some(url) = event.get_string() {
				let url_str: String = url;
				if !handler(&url_str) {
					event.event.event.veto();
				}
			}
		});
	}
	if is_url {
		web_view.load_url(url_or_content);
	} else {
		let full_html = if url_or_content.to_lowercase().contains("<html") {
			url_or_content.to_string()
		} else {
			format!("<html><head><title>{title}</title></head><body>{url_or_content}</body></html>")
		};
		web_view.set_page(&full_html, "");
	}
	let web_view_for_load = web_view;
	web_view.on_loaded(move |_| {
		// Guarded so a load that did not replace the document reuses the listener
		// already on it rather than adding a second one. A real load clears the flag
		// along with the rest of the document.
		web_view_for_load.run_script(
			"if (!window.__paperbackEscapeHooked) { \
             window.__paperbackEscapeHooked = true; \
             document.addEventListener('keydown', function(event) { \
             if (event.key === 'Escape' || event.keyCode === 27) { \
             window.wx.postMessage('close_dialog'); \
             } \
             }); \
             }",
		);
	});
	// TRANSLATORS: Label for a button that closes the Web View dialog
	let close_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Close")).build();
	let dialog_for_ok = dialog;
	let closing_for_button = Rc::clone(&closing);
	close_button.on_click(move |_| {
		if !closing_for_button.replace(true) {
			dialog_for_ok.end_modal(ID_OK);
		}
	});
	dialog.set_escape_id(ID_CANCEL);
	let sizer = BoxSizer::builder(Orientation::Vertical).build();
	sizer.add(&web_view, 1, SizerFlag::Expand | SizerFlag::All, 5);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, 5);
	sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer(sizer, true);
	dialog.centre();
	dialog.show_modal();
	ACTIVE_WEB_VIEW.with(|v| v.set(None));
}
