mod errors;

use kuchikiki::{
    NodeRef, ParseOpts, QualName,
    interface::QuirksMode,
    iter::{Descendants, Elements, NodeIterator, Select},
    local_name, namespace_url, ns, parse_fragment, parse_html, parse_html_with_options,
    tokenizer::TokenizerOpts,
    traits::TendrilSink,
    tree_builder::TreeBuilderOpts,
};
use std::error::Error;

use crate::errors::ApplicationError;

pub enum TargetCase {
    UpperCase,
    LowerCase,
}

enum InputDocumentKind {
    Document,
    Fragment,
}

impl InputDocumentKind {
    fn parse(&self, html: &str) -> NodeRef {
        match self {
            InputDocumentKind::Document => {
                // Leaving this here with options so later we can configure. All the imports are ready.
                parse_html_with_options(ParseOpts {
                    tree_builder: TreeBuilderOpts {
                        drop_doctype: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .one(html)
            }
            InputDocumentKind::Fragment => {
                let qual_name = QualName::new(None, ns!(html), local_name!(""));
                parse_fragment(qual_name, vec![]).one(html)
            }
        }
    }
}

trait TransformContents {
    fn change_case(self, selector: &str, target_case: TargetCase) -> Result<NodeRef, ApplicationError>;
}

impl TransformContents for NodeRef {
    fn change_case(self, selector: &str, target_case: TargetCase) -> Result<NodeRef, ApplicationError> {
        if let Ok(x) = self.select(selector) {
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
            Ok(self)
        } else {
            return Err(errors::ApplicationError::ParseError);
        }
    }
}

pub fn change_tag_content_case(
    html: &str,
    selector: &str,
    target_case: TargetCase,
) -> Result<String, Box<dyn Error>> {
    let current_doctype = if html.starts_with("<!DOCTYPE") {
        InputDocumentKind::Document
    } else {
        InputDocumentKind::Fragment
    };

    let trimmed_html = html.trim();
    let doc = current_doctype
        .parse(trimmed_html)
        .change_case(selector, target_case)?;


    match current_doctype {
        InputDocumentKind::Document => {
            return Ok(doc.to_string());
        },
        InputDocumentKind::Fragment => {
            // Check if the original string starts with <html> and ends with </html>
            // If it does, return doc.to_string() like is, otherwise strip these away
            // from doc.to_string().
            if trimmed_html.starts_with("<html>") && trimmed_html.ends_with("</html>") {
                return Ok(doc.to_string());
            } else {
                if let Some(val) = doc.to_string().strip_prefix("<html>"){
                    if let Some(val) = val.strip_suffix("</html>") {
                        return Ok(val.to_owned());
                    }
                }
                return Err(ApplicationError::StringManipulationError.into());
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Internals
    #[test]
    // Fragment
    fn uppercase_single_element_keep_wrapped_in_html() {
        let input_html = r"<p>hello world</p>";
        let expected_html = r"<html><p>HELLO WORLD</p></html>";

        let res = InputDocumentKind::Fragment
            .parse(input_html)
            .change_case("p", TargetCase::UpperCase)
            .expect("Should be able to parse valid HTML");


        assert_eq!(res.to_string(), expected_html);
    }

    #[test]
    // Document
    fn uppercase_document() {
        let input_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">This is the first paragraph of our example.</p><p>Here's a second paragraph, containing some more text.</p><p>This paragraph demonstrates a simple HTML structure.</p><p>Finally, this is the last paragraph in our short example.</p></body></html>";
        let expected_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">THIS IS THE FIRST PARAGRAPH OF OUR EXAMPLE.</p><p>HERE'S A SECOND PARAGRAPH, CONTAINING SOME MORE TEXT.</p><p>THIS PARAGRAPH DEMONSTRATES A SIMPLE HTML STRUCTURE.</p><p>FINALLY, THIS IS THE LAST PARAGRAPH IN OUR SHORT EXAMPLE.</p></body></html>";

    let res = InputDocumentKind::Document
            .parse(input_html)
            .change_case("p", TargetCase::UpperCase)
            .expect("Should be able to parse valid HTML");

        assert_eq!(res.to_string(), expected_html);
    }

    // External
    #[test]
    fn uppercase_single_element_only() {
        let input_html = r"<p>hello world</p>";
        let expected_html = r"<p>HELLO WORLD</p>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn uppercases_all_p_tags_with_doctype_intact() {
        let input_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">This is the first paragraph of our example.</p><p>Here's a second paragraph, containing some more text.</p><p>This paragraph demonstrates a simple HTML structure.</p><p>Finally, this is the last paragraph in our short example.</p></body></html>";
        let expected_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">THIS IS THE FIRST PARAGRAPH OF OUR EXAMPLE.</p><p>HERE'S A SECOND PARAGRAPH, CONTAINING SOME MORE TEXT.</p><p>THIS PARAGRAPH DEMONSTRATES A SIMPLE HTML STRUCTURE.</p><p>FINALLY, THIS IS THE LAST PARAGRAPH IN OUR SHORT EXAMPLE.</p></body></html>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .unwrap_or_else(|e| { e.to_string() });

        assert_eq!(res, expected_html);
    }

    #[test]
    fn uppercase_single_nested_element() {
        let input_html = r"<div><p>hello world</p></div>";
        let expected_html = r"<div><p>HELLO WORLD</p></div>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn leaves_html_tag_in_tact() {
        let input_html = r"<html><div><p>hello world</p></div></html>";
        let expected_html = r"<html><div><p>HELLO WORLD</p></div></html>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    // NOTE: These are the test cases that came with the assessment:
    #[test]
    fn uppercase_transform() {
        let input_html = r"<p>Hello world</p>";
        let expected_html = r"<p>HELLO WORLD</p>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn lowercase_transform() {
        let input_html = r"<p>Hello WORLD</p>";
        let expected_html = r"<p>hello world</p>";

        let res = change_tag_content_case(input_html, "p", TargetCase::LowerCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn multiple_paragraphs() {
        let input_html = r"<div><p>First paragraph</p><span>Not a paragraph</span><p>Second paragraph</p></div>";
        let expected_html = r"<div><p>FIRST PARAGRAPH</p><span>Not a paragraph</span><p>SECOND PARAGRAPH</p></div>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn nested_elements() {
        let input_html = r"<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>";
        let expected_html = r"<p>TEXT WITH <strong>BOLD</strong> AND <em>ITALIC</em> ELEMENTS</p>";

        let res = change_tag_content_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }
}



