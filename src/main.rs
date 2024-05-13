use std::fs;
use crate::lexemes::TokenType;

mod cleanup;
mod lexemes;

fn main() {
    let basic = fs::read_to_string("data/fish_counter.cpp").unwrap();
    let cleaned = cleanup::remove_comments(basic);
    let lexemes = lexemes::count_tokens(cleaned);
    let tmp = lexemes.unwrap();
    // println!("{:#?}", lexemes.unwrap());
    let words: Vec<_> = tmp.iter().filter(|t| t.token_type == TokenType::StringLiteral).collect();
    for i in words {
        println!("{}", i.token)
    }
}
