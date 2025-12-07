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

    text.trim().to_string()
}
