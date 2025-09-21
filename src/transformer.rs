// transformer.rs
// MarkdownElement → HTML 인라인 변환

use regex::Regex;
use crate::parser::MarkdownElement;

pub fn transform_element(element: &MarkdownElement) -> String {
    match element {
        MarkdownElement::Header { level, content } => {
            let text = transform_inline(content);
            format!("<h{level}>{text}</h{level}>", level = level, text = text)
        }
        MarkdownElement::Paragraph(content) => {
            let text = transform_inline(content);
            format!("<p>{}</p>", text)
        }
        MarkdownElement::BlankLine => "<br/>".to_string(),
    }
}

/// 인라인 변환 함수
fn transform_inline(text: &str) -> String {
    let mut s = text.to_string();

    // HTML 특수 문자 이스케이프
    s = s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");

    // inline code `` `code` ``
    let re_code = Regex::new(r"`([^`]+)`").unwrap();
    s = re_code.replace_all(&s, "<code>$1</code>").to_string();

    // bold **text**
    let re_bold = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    s = re_bold.replace_all(&s, "<strong>$1</strong>").to_string();

    // italic *text*
    let re_italic = Regex::new(r"\*(.+?)\*").unwrap();
    s = re_italic.replace_all(&s, "<em>$1</em>").to_string();

    // link [text](url)
    let re_link = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    s = re_link.replace_all(&s, "<a href=\"$2\">$1</a>").to_string();

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::MarkdownElement;

    #[test]
    fn test_transform_inline() {
        let md = "This is **bold** text and *italic* text with `code` and [link](http://example.com).";
        let transformed = transform_inline(md);
        assert!(transformed.contains("<strong>bold</strong>"));
        assert!(transformed.contains("<em>italic</em>"));
        assert!(transformed.contains("<code>code</code>"));
        assert!(transformed.contains("<a href=\"http://example.com\">link</a>"));
    }

    #[test]
    fn test_transform_element() {
        let elem = MarkdownElement::Header { level: 1, content: "**Hello** World".to_string() };
        let html = transform_element(&elem);
        assert_eq!(html, "<h1><strong>Hello</strong> World</h1>");
    }
}
