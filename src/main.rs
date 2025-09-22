// ============= 안승우 파트 =============

mod parser; //parser.rs 가져오기
mod transformer; //transformer.rs 가져오기
mod codegen; //codegen.rs 가져오기

use std::env; //명령값 인자 받을 떄 사용
use std::fs; // 파일 읽고 쓸때 사용
use std::process; //프로그램 종료할때 사용

fn main(){
    let args: Vec<String> = env::args().collect(); //명령값 인자 다 받아서 Vec에 저장

    if args.len() < 3 { //args의 길이가 3 미만이면 (명령값 인자가 부족하면)
        eprintln!("사용법 : {} <yourfile.md> <output.html>", args[0]); //사용법 출력
        process::exit(1); //프로그램 종료 코드 1 반환
    }

    let mdfile = &args[1]; //md 파일 저장
    let htmlfile = &args[2]; //html 파일 저장

    let md = match fs::read_to_string(mdfile) { //mdfile 읽어서 md에 저장
        Ok(text) => text, //쭉 흘러가면 그 안에 내용 저장
        Err(err) => { // 에러가 나면
            eprintln!("입력 파일 읽기 오류!! : {}", err); //입력 파일 읽기 오류와 에러 내용 출력
            process::exit(1); //프로그램 종료 코드 1 반환
        }
    };

    let elements = parser::do_parse(&md); //파싱
    let html = codegen::generate_html(elements); //html 코드 변환

    if let Err(err) = fs::write(htmlfile, html) {
        eprintln!("출력 파일 저장 오류 : {}", err);
        process::exit(1);
    }

    println!("컴파일 성공 : {} -> {}", mdfile, htmlfile);







}