//! The Documents tab: how a file is read. Changing any of these reads open documents again.

use paperback_core::{config::ConfigManager, document::PageLayout, parser::braille};
use patois::t;
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSettings {
	pub render_tables_inline: bool,
	pub page_layout: PageLayout,
	pub strip_running_text: bool,
	pub markdown_dollar_math: bool,
	pub braille_table: &'static str,
	pub keep_braille: bool,
}

impl DocumentSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			render_tables_inline: config.get_app_bool("render_tables_inline", true),
			page_layout: config.get_page_layout(),
			strip_running_text: config.get_app_bool("strip_running_text", true),
			markdown_dollar_math: config.get_app_bool("markdown_dollar_math", true),
			braille_table: braille::chosen_table(&config.get_app_string("braille_table", braille::DEFAULT_TABLE)),
			keep_braille: config.get_app_bool("keep_braille", false),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		config.set_app_bool("render_tables_inline", self.render_tables_inline);
		config.set_page_layout(self.page_layout);
		config.set_app_bool("strip_running_text", self.strip_running_text);
		config.set_app_bool("markdown_dollar_math", self.markdown_dollar_math);
		config.set_app_string("braille_table", self.braille_table);
		config.set_app_bool("keep_braille", self.keep_braille);
	}
}

fn layout_label(layout: PageLayout) -> String {
	match layout {
		// TRANSLATORS: Page layout choice in the Documents tab of Settings: PDF and XPS pages are read in the order the document gives, from its tags where it has them, with wrapped lines joined into paragraphs
		PageLayout::Automatic => t("Automatic"),
		// TRANSLATORS: Page layout choice in the Documents tab of Settings: the document's own reading order is ignored and the columns of each page are found from where the text sits, then read one after the other. It suits documents whose text is read out of order, such as two-column layouts
		PageLayout::Columns => t("By columns, ignoring the document's own order"),
		// TRANSLATORS: Page layout choice in the Documents tab of Settings: like reading by columns, but every line is kept apart instead of joined into paragraphs, which suits code, poetry and lists of clauses whose line breaks are the content
		PageLayout::Lines => t("By columns, keeping every line apart"),
	}
}

pub(super) struct DocumentsTab {
	render_tables_inline: CheckBox,
	page_layout: Choice,
	strip_running_text: CheckBox,
	markdown_dollar_math: CheckBox,
	braille_table: Choice,
	keep_braille: CheckBox,
}

impl DocumentsTab {
	pub(super) fn new(panel: Panel, settings: &DocumentSettings, padding: i32) -> Self {
		// TRANSLATORS: Note at the top of the Documents tab of Settings, whose options change how a file is read
		let note_text = t("Changing these reads your open documents again when you press OK.");
		let note = StaticText::builder(&panel).with_label(&note_text).build();
		// A screen reader skips text that can't take focus, but reads a property page's description as focus moves into it, as NVDA's own settings panels do.
		panel.set_accessibility_description(&note_text);
		let render_tables_inline =
			// TRANSLATORS: Option to render tables inline rather than showing a placeholder link
			CheckBox::builder(&panel).with_label(&t("Render tables &inline")).build();
		// TRANSLATORS: Label for the dropdown choosing how the pages of PDF and XPS documents are put in reading order and paragraphs, for every document not given a page layout of its own in the View menu
		let page_layout_label_text = t("Page &layout of PDF and XPS documents:");
		let page_layout_label = StaticText::builder(&panel).with_label(&page_layout_label_text).build();
		let page_layout = Choice::builder(&panel).build();
		for layout in PageLayout::ALL {
			page_layout.append(&layout_label(layout));
		}
		#[cfg(target_os = "macos")]
		page_layout.set_accessibility_label(page_layout_label_text.replace('&', "").trim_end_matches(':').trim());
		let strip_running_text = CheckBox::builder(&panel)
			// TRANSLATORS: Option to take the headers and footers a document repeats on every page out of the text, in documents laid out as pages, such as PDFs. Turning it off keeps them, which suits a document whose page-edge lines are content.
			.with_label(&t("Remove repeated page &headers and footers"))
			.build();
		let markdown_dollar_math = CheckBox::builder(&panel)
			// TRANSLATORS: Option to read text between dollar signs in Markdown documents, such as $x^2$, as a math formula. Turning it off keeps the dollar signs as plain text, which suits documents that use them for other things, such as shell variables or prices.
			.with_label(&t("Read text between dollar signs as &math in Markdown"))
			.build();
		// TRANSLATORS: Label for the dropdown choosing the braille code that Braille Ready Files (BRF) and PEF books are written in, so Paperback can turn their braille back into print
		let braille_table_label_text = t("&Braille code of BRF and PEF books:");
		let braille_table_label = StaticText::builder(&panel).with_label(&braille_table_label_text).build();
		let braille_table = Choice::builder(&panel).build();
		// Frozen because on Windows each append resizes the drop-down, which made Settings take two seconds to open with 322 tables; thawing resizes it once.
		braille_table.freeze();
		for table in braille::braille_tables() {
			braille_table.append(&table.label());
		}
		braille_table.thaw();
		#[cfg(target_os = "macos")]
		braille_table.set_accessibility_label(braille_table_label_text.replace('&', "").trim_end_matches(':').trim());
		let keep_braille = CheckBox::builder(&panel)
			// TRANSLATORS: Option to show BRF, PEF, eBraille and BARD braille books as their braille, in Unicode braille, instead of turning them into print. It suits reading on a braille display, where the screen reader would otherwise turn the print back into braille; speech reads the cells as braille patterns. The braille code above still says which braille the books are written in.
			.with_label(&t("&Keep braille books in braille, for reading on a braille display"))
			.build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&note, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&render_tables_inline, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(page_layout_label, &page_layout, padding), 0, SizerFlag::All, OPTION_PADDING);
		for check in [&strip_running_text, &markdown_dollar_math] {
			sizer.add(check, 0, SizerFlag::All, OPTION_PADDING);
		}
		sizer.add_sizer(&labeled_row(braille_table_label, &braille_table, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&keep_braille, 0, SizerFlag::All, OPTION_PADDING);
		panel.set_sizer(sizer, true);
		render_tables_inline.set_value(settings.render_tables_inline);
		let layout_index =
			PageLayout::ALL.iter().position(|layout| *layout == settings.page_layout).unwrap_or_default();
		page_layout.set_selection(u32::try_from(layout_index).unwrap_or_default());
		strip_running_text.set_value(settings.strip_running_text);
		markdown_dollar_math.set_value(settings.markdown_dollar_math);
		let table_index = braille::braille_tables()
			.iter()
			.position(|table| table.file_name == settings.braille_table)
			.unwrap_or_default();
		braille_table.set_selection(u32::try_from(table_index).unwrap_or_default());
		keep_braille.set_value(settings.keep_braille);
		Self {
			render_tables_inline,
			page_layout,
			strip_running_text,
			markdown_dollar_math,
			braille_table,
			keep_braille,
		}
	}

	pub(super) fn values(&self) -> DocumentSettings {
		DocumentSettings {
			render_tables_inline: self.render_tables_inline.is_checked(),
			page_layout: self
				.page_layout
				.get_selection()
				.and_then(|index| PageLayout::ALL.get(index as usize).copied())
				.unwrap_or_default(),
			strip_running_text: self.strip_running_text.is_checked(),
			markdown_dollar_math: self.markdown_dollar_math.is_checked(),
			braille_table: self
				.braille_table
				.get_selection()
				.and_then(|index| braille::braille_tables().get(index as usize))
				.map_or(braille::DEFAULT_TABLE, |table| table.file_name),
			keep_braille: self.keep_braille.is_checked(),
		}
	}
}
