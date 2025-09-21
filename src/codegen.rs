// src/codegen.rs
use crate::parser::MarkdownElement;
use crate::transformer;

/// MarkdownElement 리스트를 HTML 문자열로 변환
pub fn generate_html(elements: Vec<MarkdownElement>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Markdown</title>\n</head>\n<body>\n");

    for elem in elements {
        html.push_str(&transformer::transform_element(&elem));
        html.push_str("\n");
    }

    html.push_str("</body>\n</html>\n");
    html
}
