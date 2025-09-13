use kuchikiki::{
    NodeRef, ParseOpts, QualName, iter::NodeIterator, local_name, namespace_url, ns,
    parse_fragment, parse_html_with_options, traits::TendrilSink, tree_builder::TreeBuilderOpts,
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
                // NOTE: Leaving this here with options so later we can configure. All the imports are ready.
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
    fn change_case(
        self,
        selector: &str,
        target_case: TargetCase,
    ) -> Result<NodeRef, ApplicationError>;
}

impl TransformContents for NodeRef {
    fn change_case(
        self,
        selector: &str,
        target_case: TargetCase,
    ) -> Result<NodeRef, ApplicationError> {
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
            return Err(ApplicationError::ParseError);
        }
    }
}

/** 
* Transforms the case of an HTML string */
pub fn transform_case(
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
        }
        InputDocumentKind::Fragment => {
            // Check if the original string starts with <html> and ends with </html>
            // If it does, return doc.to_string() like is, otherwise strip these away
            // from doc.to_string().
            if trimmed_html.starts_with("<html>") && trimmed_html.ends_with("</html>") {
                return Ok(doc.to_string());
            } else {
                if let Some(val) = doc.to_string().strip_prefix("<html>") {
                    if let Some(val) = val.strip_suffix("</html>") {
                        return Ok(val.to_owned());
                    }
                }
                return Err(ApplicationError::StringManipulationError.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Internals
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

    // NOTE: External interface
    #[test]
    fn uppercases_all_p_tags_with_doctype_intact() {
        let input_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">This is the first paragraph of our example.</p><p>Here's a second paragraph, containing some more text.</p><p>This paragraph demonstrates a simple HTML structure.</p><p>Finally, this is the last paragraph in our short example.</p></body></html>";
        let expected_html = "<!DOCTYPE html><html><head><title>Simple Paragraph Example</title></head><body><p class=\"asd\">THIS IS THE FIRST PARAGRAPH OF OUR EXAMPLE.</p><p>HERE'S A SECOND PARAGRAPH, CONTAINING SOME MORE TEXT.</p><p>THIS PARAGRAPH DEMONSTRATES A SIMPLE HTML STRUCTURE.</p><p>FINALLY, THIS IS THE LAST PARAGRAPH IN OUR SHORT EXAMPLE.</p></body></html>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .unwrap_or_else(|e| e.to_string());

        assert_eq!(res, expected_html);
    }

    #[test]
    fn uppercase_single_nested_element() {
        let input_html = r"<div><p>hello world</p></div>";
        let expected_html = r"<div><p>HELLO WORLD</p></div>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn leaves_html_tag_in_tact() {
        let input_html = r"<html><div><p>hello world</p></div></html>";
        let expected_html = r"<html><div><p>HELLO WORLD</p></div></html>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn adjacent_tags_handled_fine() {
        let input_html = r"<span>hey</span><p>Hello, hoWsit?</p>";
        let expected_html = r"<span>hey</span><p>HELLO, HOWSIT?</p>";
  
        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn multiple_selectors_handled_fine() {
        let input_html = "<div class=\"asd\">tag</div><span>hEy</span><p>Hello, hoWsit?</p>";
        let expected_html = "<div class=\"asd\">tag</div><span>HEY</span><p>HELLO, HOWSIT?</p>";

        let res = transform_case(input_html, "span,p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }
    #[test]
    fn can_use_class_selectors() {
        let input_html = "<div class=\"asd\">tag</div><span>hEy</span><p>Hello, hoWsit?</p>";
        let expected_html = "<div class=\"asd\">TAG</div><span>hEy</span><p>Hello, hoWsit?</p>";

        let res = transform_case(input_html, ".asd", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }
}

