// src/main.rs
mod parser;
mod transformer;
mod codegen;

use std::env;
use std::fs;
use parser::MarkdownElement;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.md> [output.html]", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = if args.len() >= 3 { &args[2] } else { "output.html" };

    let md_content = fs::read_to_string(input_file).expect("Cannot read input file");

    // 1. Parser
    let parsed: Vec<MarkdownElement> = parser::parse_lines(&md_content);

    // 2. CodeGen + Transformer 연결
    let html_content = codegen::generate_html(parsed);

    fs::write(output_file, html_content).expect("Cannot write output file");

    println!("Conversion complete: {} → {}", input_file, output_file);
}
