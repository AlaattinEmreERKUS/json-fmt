use std::fmt::Display;

#[derive(Debug)]
enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(String),
    True,
    False,
    Null,
}

fn main() {
    let input = String::from("{,}");
    lexer(input);
}

fn lexer(input: String) {
    for (_i, byte) in input.into_bytes().into_iter().enumerate() {
        let char = char::from(byte);
        match char {
            '{' => println!("{:?}", Token::LeftBrace),
            '}' => println!("{:?}", Token::RightBrace),
            '[' => println!("{:?}", Token::LeftBracket),
            ']' => println!("{:?}", Token::RightBracket),
            ',' => println!("{:?}", Token::Comma),
            ':' => println!("{:?}", Token::Colon),
            _ => println!("{:?}", Token::String(char.to_string())),
        }
    }
}
