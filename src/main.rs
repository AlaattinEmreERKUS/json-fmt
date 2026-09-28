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
                let end = literal_to_token(&chars, &['t', 'r', 'u', 'e'], i)?;
                tokens.push(Token::True);
                i = end;
                continue;
            }
            'f' => {
                let end = literal_to_token(&chars, &['f', 'a', 'l', 's', 'e'], i)?;
                tokens.push(Token::False);
                i = end;
                continue;
            }
            'n' => {
                let end = literal_to_token(&chars, &['n', 'u', 'l', 'l'], i)?;
                tokens.push(Token::Null);
                i = end;
                continue;
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

fn literal_kontrol(chars: &[char], word: &[char], i: usize) -> Result<usize, String> {
    if chars[i..].starts_with(word) {
        let maybe = chars.get(i + word.len());
        match maybe {
            Some(c) => {
                if !STRUCTURAL.contains(&c) && !WHITESPACE.contains(&c) {
                    return Err(format!(
                        "Hatalı karakter {}, konum: {}",
                        c,
                        i + word.len() + 1
                    ));
                }
            }
            None => {}
        }
        Ok(i + word.len())
    } else {
        Err(format!(
            "{} bekleniyordu, konum: {}",
            word.iter().collect::<String>(),
            i + 1
        ))
    }
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
    fn test_true_boundary_in_middle() {
        assert_eq!(
            tokenizer("[true, truex]"),
            Err("Hatalı karakter x, konum: 12".to_string())
        );
    }

    #[test]
    fn test_null_tokenizer_over_size() {
        assert_eq!(
            tokenizer("nulls"),
            Err("Hatalı karakter s, konum: 5".to_string())
        );
    }

    #[test]
    fn test_false_tokenizer_over_size() {
        assert_eq!(
            tokenizer("falseds"),
            Err("Hatalı karakter d, konum: 6".to_string())
        );
    }

    #[test]
    fn test_tokenizer_same_literal_twice() {
        assert_eq!(
            tokenizer("truetrue"),
            Err("Hatalı karakter t, konum: 5".to_string())
        );
    }

    #[test]
    fn test_tokenizer_literal_with_number_ending() {
        assert_eq!(
            tokenizer("true123"),
            Err("Hatalı karakter 1, konum: 5".to_string())
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
