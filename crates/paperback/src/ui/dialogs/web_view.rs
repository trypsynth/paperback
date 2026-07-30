use patois::t;
use wxdragon::{event::WebViewEvents, prelude::*, widgets::WebView};

type NavigationHandler = Box<dyn Fn(&str) -> bool>;
use std::cell::Cell;

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
		.with_size(800, 600)
		.build();
	ACTIVE_WEB_VIEW.with(|v| v.set(Some(dialog)));
	let web_view = WebView::builder(&dialog).build();
	web_view.add_script_message_handler("wx");
	let dialog_for_close = dialog;
	web_view.on_script_message_received(move |event| {
		if event.get_string() == Some("close_dialog".to_string()) {
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
		web_view_for_load.run_script(
			"document.addEventListener('keydown', function(event) { \
             if (event.key === 'Escape' || event.keyCode === 27) { \
             window.wx.postMessage('close_dialog'); \
             } \
             });",
		);
	});
	let close_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Close")).build();
	let dialog_for_ok = dialog;
	close_button.on_click(move |_| {
		dialog_for_ok.end_modal(ID_OK);
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
