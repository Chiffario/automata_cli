use std::fs;

mod cleanup;
mod lexemes;

fn main() {
    let basic = fs::read_to_string("data/fuzz.cpp").unwrap();
    let cleaned = cleanup::remove_comments(basic);
    println!("{}", cleanup::add_line_numbers(cleaned.clone()));
    let lexemes = lexemes::count_tokens(cleaned);
    let tmp = lexemes.unwrap();
    println!("{:#?}", tmp);
}
