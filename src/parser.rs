// parser.rs
// Markdown → 구조화된 라인 리스트로 변환

#[derive(Debug, Clone)]
pub enum MarkdownElement {
    Header { level: u8, content: String },
    Paragraph(String),
    BlankLine,
}

pub fn parse_lines(md: &str) -> Vec<MarkdownElement> {
    let mut elements = Vec::new();

    for line in md.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            elements.push(MarkdownElement::BlankLine);
        } else if trimmed.starts_with("### ") {
            elements.push(MarkdownElement::Header {
                level: 3,
                content: trimmed[4..].to_string(),
            });
        } else if trimmed.starts_with("## ") {
            elements.push(MarkdownElement::Header {
                level: 2,
                content: trimmed[3..].to_string(),
            });
        } else if trimmed.starts_with("# ") {
            elements.push(MarkdownElement::Header {
                level: 1,
                content: trimmed[2..].to_string(),
            });
        } else {
            elements.push(MarkdownElement::Paragraph(trimmed.to_string()));
        }
    }

    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let md = r#"
# Title

This is a paragraph.

## Subtitle
"#;
        let result = parse_lines(md);

        assert_eq!(result.len(), 5);
        match &result[1] {
            MarkdownElement::Paragraph(text) => assert_eq!(text, "This is a paragraph."),
            _ => panic!("Expected paragraph"),
        }
        match &result[3] {
            MarkdownElement::Header { level, content } => {
                assert_eq!(*level, 2);
                assert_eq!(content, "Subtitle");
            }
            _ => panic!("Expected header level 2"),
        }
    }
}
