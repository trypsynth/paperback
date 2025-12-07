use pulldown_cmark::{Event, Parser, TagEnd};

pub fn markdown_to_text(markdown: &str) -> String {
	let mut text = String::new();
	let parser = Parser::new(markdown);

	for event in parser {
		match event {
			Event::Text(t) => {
				text.push_str(&t);
			}
			Event::End(TagEnd::Paragraph) => {
				text.push_str("\n\n");
			}
			Event::End(TagEnd::Heading(_)) => {
				text.push_str("\n\n");
			}
			Event::End(TagEnd::Item) => {
				text.push('\n');
			}
			_ => {}
		}
	}

	let mut result = format!(" {}", text.trim());
	loop {
		let original_len = result.len();
		if let Some(start) = result.find(" #") {
			if let Some(substr) = result.get(start + 2..) {
				let num_len = substr.chars().take_while(|c| c.is_ascii_digit()).count();
				if num_len > 0 {
					let mut end = start + 2 + num_len;
					if let Some(after_num) = result.get(end..) {
						if after_num.starts_with(',') {
							end += 1;
						} else if after_num.starts_with('.') {
							if after_num.get(1..).map_or(true, |s| s.starts_with(char::is_whitespace)) {
								end += 1;
							}
						}
					}
					result.replace_range(start..end, "");
				}
			}
		}
		if result.len() == original_len {
			break;
		}
	}

	result.trim_start().to_string()
}
