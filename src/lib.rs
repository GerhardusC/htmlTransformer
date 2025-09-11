mod errors;

use std::error::Error;
use kuchikiki::{
    iter::NodeIterator, parse_html, traits::TendrilSink,
};

pub enum TargetCase {
    UpperCase,
    LowerCase,
}

pub fn change_tag_content_case(
    html: &str,
    selector: &str,
    target_case: TargetCase,
) -> Result<String, Box<dyn Error>> {
    let doc = parse_html().one(html);
    match doc.select(selector) {
        Ok(x) => {
            x.for_each(|x| {
                x.as_node()
                    .descendants()
                    .text_nodes()
                    .for_each(|text_cell| {
                        let new_val = match target_case {
                            TargetCase::UpperCase => text_cell.borrow().to_uppercase(),
                            TargetCase::LowerCase => text_cell.borrow().to_lowercase(),
                        };
                        *text_cell.borrow_mut() = new_val;
                    });
            });
        }
        Err(_) => {
            return Err(errors::ApplicationError::ParseError.into());
        }
    };

    return Ok(doc.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_single_element() {
        let input_html = r"<p>hello world</p>";
        let expected_html = r"<html><head></head><body><p>HELLO WORLD</p></body></html>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn uppercases_all_p_tags() {
        let input_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">This is the first paragraph of our example.</p><p>Here's a second paragraph, containing some more text.</p><p>This paragraph demonstrates a simple HTML structure.</p><p>Finally, this is the last paragraph in our short example.</p></body></html>";
        let expected_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">THIS IS THE FIRST PARAGRAPH OF OUR EXAMPLE.</p><p>HERE'S A SECOND PARAGRAPH, CONTAINING SOME MORE TEXT.</p><p>THIS PARAGRAPH DEMONSTRATES A SIMPLE HTML STRUCTURE.</p><p>FINALLY, THIS IS THE LAST PARAGRAPH IN OUR SHORT EXAMPLE.</p></body></html>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse valid HTML");

        assert_eq!(res, expected_html);
    }
}
