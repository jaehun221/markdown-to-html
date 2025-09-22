// ============= 이재훈 파트 =============

use regex::Regex;
use crate::parser::MarkdownElement;

pub fn transform_element(element: &MarkdownElement) -> String {
    match element {
        MarkdownElement::Header { count, text } => {
            let content = transform_inline(text);
            format!("<h{count}>{content}</h{count}", count = count, content = content)
        }
        MarkdownElement::Ptag(text) => {
            let content = transform_inline(text);
            format!("<p>{}</p>", content)
        }
        MarkdownElement::BlankLine => "<br/>".to_string(),
    }
}


fn transform_inline(text: &str) -> String {
    let mut s =  text.to_string();

    // HTML 이스케이프 문자
    s = s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");


    // inline code ```code```
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