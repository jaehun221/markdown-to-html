// ============= 안승우 파트 =============

//내용 설명 : 마크 다운 파일에 일반 단락과 빈칸 그리고 헤더를 나눠줌 (줄마다) 헤더는 #의 갯수에 따라 h1, h2, h3로 나눠줌


// ============= 마크 다운의 3요소 =============
#[derive(Debug, Clone)]
pub enum MarkdownElement {
    Header{ count: u8, text: String }, // h1 = count 1, h2 = count 2, h3 = count 3
    Ptag(String), //p태그
    BlankLine, //빈칸
}

// ============= 마크 다운 파싱 함수 =============
pub fn do_parse(md: &str) -> Vec<MarkdownElement> {
    let mut elements = Vec::new();

    for line in md.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            elements.push(MarkdownElement::BlankLine);
        } else if trimmed_line.starts_with("###"){
            elements.push(MarkdownElement::Header{
                count: 3,
                text: trimmed_line[4..].to_string()
            })
        } else if trimmed_line.starts_with("##"){
            elements.push(MarkdownElement::Header{
                count: 2,
                text: trimmed_line[3..].to_string()
            })
        } else if trimmed_line.starts_with("#"){
            elements.push(MarkdownElement::Header{
                count: 1,
                text: trimmed_line[2..].to_string()
            })
        } else {
            elements.push(MarkdownElement::Ptag(trimmed_line.to_string()));
        }
    }

    elements
}

// ============= 출력 예시 =============
// ============= example.md =============
// # Header 1
// This is a paragraph.
// ## Header 2
// Another paragraph.
// ### Header 3
// ============= 출력 =============
// [Header { count: 1, text: "Header 1" },
//  Ptag("This is a paragraph."),
//  Header { count: 2, text: "Header 2" },
//  Ptag("Another paragraph."),
//  Header { count: 3, text: "Header 3" }]