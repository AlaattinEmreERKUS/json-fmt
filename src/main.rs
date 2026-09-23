use std::{format, println};

#[derive(Debug, PartialEq)]
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

const STRUCTURAL: &[char] = &['{', '}', '[', ']', ',', ':'];
const WHITESPACE: &[char] = &[' ', '\t', '\n', '\r'];

fn main() {
    let input = String::from("");
    let tokens = tokenizer(&input);
    println!("{:?}", tokens);
}

fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    while i < chars.len() {
        let character = chars[i];
        match character {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            't' => {
                if chars[i..].starts_with(&['t', 'r', 'u', 'e']) {
                    let maybe = chars.get(i + 4);
                    match maybe {
                        Some(c) => {
                            if !STRUCTURAL.contains(&c) && !WHITESPACE.contains(&c) {
                                return Err(format!("true bekleniyordu, konum: {}", i + 1));
                            }
                        }
                        None => {}
                    }
                    tokens.push(Token::True);
                    i += 4;
                    continue;
                } else {
                    return Err(format!("true bekleniyordu, konum: {}", i + 1));
                }
            }
            'f' => {
                if chars[i..].starts_with(&['f', 'a', 'l', 's', 'e']) {
                    let maybe = chars.get(i + 5);
                    match maybe {
                        Some(c) => {
                            if !STRUCTURAL.contains(&c) && !WHITESPACE.contains(&c) {
                                return Err(format!("false bekleniyordu, konum: {}", i + 1));
                            }
                        }
                        None => {}
                    }
                    tokens.push(Token::False);
                    i += 5;
                    continue;
                } else {
                    return Err(format!("false bekleniyordu, konum: {}", i + 1));
                }
            }
            'n' => {
                if chars[i..].starts_with(&['n', 'u', 'l', 'l']) {
                    let maybe = chars.get(i + 4);
                    match maybe {
                        Some(c) => {
                            if !STRUCTURAL.contains(&c) && !WHITESPACE.contains(&c) {
                                return Err(format!("null bekleniyordu, konum: {}", i + 1));
                            }
                        }
                        None => {}
                    }
                    tokens.push(Token::Null);
                    i += 4;
                    continue;
                } else {
                    return Err(format!("null bekleniyordu, konum: {}", i + 1));
                }
            }
            ' ' | '\n' | '\t' | '\r' => {
                i += 1;
                continue;
            }
            _ => {
                return Err(format!(
                    "Yanlış karakter kullanımı: {}, konum: {}",
                    character,
                    i + 1
                ));
            }
        }
        i += 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_three_literal() {
        assert_eq!(
            tokenizer("true false null"),
            Ok(vec![Token::True, Token::False, Token::Null])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_left_brace() {
        assert_eq!(
            tokenizer("true false null {"),
            Ok(vec![
                Token::True,
                Token::False,
                Token::Null,
                Token::LeftBrace
            ])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_right_brace() {
        assert_eq!(
            tokenizer("true false null }"),
            Ok(vec![
                Token::True,
                Token::False,
                Token::Null,
                Token::RightBrace
            ])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_right_bracket() {
        assert_eq!(
            tokenizer("true false null ]"),
            Ok(vec![
                Token::True,
                Token::False,
                Token::Null,
                Token::RightBracket
            ])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_comma() {
        assert_eq!(
            tokenizer("true false null,"),
            Ok(vec![Token::True, Token::False, Token::Null, Token::Comma])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_colon() {
        assert_eq!(
            tokenizer("true false null:"),
            Ok(vec![Token::True, Token::False, Token::Null, Token::Colon])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_left_bracket() {
        assert_eq!(
            tokenizer("true false null ["),
            Ok(vec![
                Token::True,
                Token::False,
                Token::Null,
                Token::LeftBracket
            ])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_space() {
        assert_eq!(
            tokenizer("true false null "),
            Ok(vec![Token::True, Token::False, Token::Null,])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_backslash_n() {
        assert_eq!(
            tokenizer("true false null \n"),
            Ok(vec![Token::True, Token::False, Token::Null])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_backslash_r() {
        assert_eq!(
            tokenizer("true false null \r"),
            Ok(vec![Token::True, Token::False, Token::Null])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_backslash_r_n() {
        assert_eq!(
            tokenizer("true false null \r\n"),
            Ok(vec![Token::True, Token::False, Token::Null])
        );
    }

    #[test]
    fn test_tokenizer_finishing_with_backslash_t() {
        assert_eq!(
            tokenizer("true false null \t"),
            Ok(vec![Token::True, Token::False, Token::Null])
        );
    }

    #[test]
    fn test_tokenizer_over_size() {
        assert_eq!(
            tokenizer("truelds"),
            Err("true bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_same_literal_twice() {
        assert_eq!(
            tokenizer("truetrue"),
            Err("true bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_literal_with_number_ending() {
        assert_eq!(
            tokenizer("true123"),
            Err("true bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_not_finished_true() {
        assert_eq!(
            tokenizer("tru"),
            Err("true bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_not_finished_false() {
        assert_eq!(
            tokenizer("fals"),
            Err("false bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_not_finished_null() {
        assert_eq!(
            tokenizer("nul"),
            Err("null bekleniyordu, konum: 1".to_string())
        );
    }

    #[test]
    fn test_tokenizer_space() {
        assert_eq!(tokenizer(" "), Ok(vec![]));
    }

    #[test]
    fn test_tokenizer_nothing() {
        assert_eq!(tokenizer(""), Ok(vec![]));
    }

    #[test]
    fn test_tokenizer_starting_with_space() {
        assert_eq!(tokenizer(" true"), Ok(vec![Token::True]));
    }

    #[test]
    fn test_tokenizer_randomized_valid_json() {
        assert_eq!(
            tokenizer("[true,null:{false}]"),
            Ok(vec![
                Token::LeftBracket,
                Token::True,
                Token::Comma,
                Token::Null,
                Token::Colon,
                Token::LeftBrace,
                Token::False,
                Token::RightBrace,
                Token::RightBracket
            ])
        );
    }
}
