// NOTE: These are the test cases from the assessment, more test cases on the internals can
// be found in `parsing.rs`.
#[cfg(test)]
mod tests {
    use crate::parsing::{transform_case, TargetCase};
    #[test]
    fn uppercase_transform() {
        let input_html = r"<p>Hello world</p>";
        let expected_html = r"<p>HELLO WORLD</p>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn lowercase_transform() {
        let input_html = r"<p>Hello WORLD</p>";
        let expected_html = r"<p>hello world</p>";

        let res = transform_case(input_html, "p", TargetCase::LowerCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn multiple_paragraphs() {
        let input_html =
            r"<div><p>First paragraph</p><span>Not a paragraph</span><p>Second paragraph</p></div>";
        let expected_html =
            r"<div><p>FIRST PARAGRAPH</p><span>Not a paragraph</span><p>SECOND PARAGRAPH</p></div>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }

    #[test]
    fn nested_elements() {
        let input_html = r"<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>";
        let expected_html = r"<p>TEXT WITH <strong>BOLD</strong> AND <em>ITALIC</em> ELEMENTS</p>";

        let res = transform_case(input_html, "p", TargetCase::UpperCase)
            .expect("Should be able to parse msg");

        assert_eq!(res, expected_html);
    }
}
