use std::{format, println};

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
    let input = String::from("false, true:null{null}, true:{false}{true,:[]false}}");
    let tokens = tokenizer(input);
    println!("{:?}", tokens);
}

fn tokenizer(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    while i < chars.len() {
        println!("{}", i.to_string());
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
                    if chars.get(i + 4) != Some(&'{')
                        && chars.get(i + 4) != Some(&'}')
                        && chars.get(i + 4) != Some(&'[')
                        && chars.get(i + 4) != Some(&']')
                        && chars.get(i + 4) != Some(&',')
                        && chars.get(i + 4) != Some(&':')
                        && chars.get(i + 4) != Some(&'\n')
                        && chars.get(i + 4) != Some(&'\t')
                        && chars.get(i + 4) != Some(&' ')
                        && chars.get(i + 4) != Some(&'\r')
                        && chars.get(i + 4) != None
                    {
                        return Err(format!(
                            "Yanlış karakter kullanımı: {:?}, konum: {}",
                            chars.get(i + 4),
                            i + 1
                        ));
                    }
                    tokens.push(Token::True);
                    i += 4;
                    continue;
                } else {
                    return Err(format!(
                        "Yanlış karakter kullanımı: {:?}, konum: {}",
                        chars.get(i + 4),
                        i + 1
                    ));
                }
            }
            'f' => {
                if chars[i..].starts_with(&['f', 'a', 'l', 's', 'e']) {
                    if chars.get(i + 5) != Some(&'{')
                        && chars.get(i + 5) != Some(&'}')
                        && chars.get(i + 5) != Some(&'[')
                        && chars.get(i + 5) != Some(&']')
                        && chars.get(i + 5) != Some(&',')
                        && chars.get(i + 5) != Some(&':')
                        && chars.get(i + 5) != Some(&'\n')
                        && chars.get(i + 5) != Some(&'\t')
                        && chars.get(i + 5) != Some(&' ')
                        && chars.get(i + 5) != Some(&'\r')
                        && chars.get(i + 5) != None
                    {
                        return Err(format!(
                            "Yanlış karakter kullanımı: {:?}, konum: {}",
                            chars.get(i + 5),
                            i + 1
                        ));
                    }
                    tokens.push(Token::False);
                    i += 5;
                    continue;
                } else {
                    return Err(format!(
                        "Yanlış karakter kullanımı: {:?}, konum: {}",
                        chars.get(i + 5),
                        i + 1
                    ));
                }
            }
            'n' => {
                if chars[i..].starts_with(&['n', 'u', 'l', 'l']) {
                    if chars.get(i + 4) != Some(&'{')
                        && chars.get(i + 4) != Some(&'}')
                        && chars.get(i + 4) != Some(&'[')
                        && chars.get(i + 4) != Some(&']')
                        && chars.get(i + 4) != Some(&',')
                        && chars.get(i + 4) != Some(&':')
                        && chars.get(i + 4) != Some(&'\n')
                        && chars.get(i + 4) != Some(&'\t')
                        && chars.get(i + 4) != Some(&' ')
                        && chars.get(i + 4) != Some(&'\r')
                        && chars.get(i + 4) != None
                    {
                        return Err(format!(
                            "Yanlış karakter kullanımı: {:?}, konum: {}",
                            chars.get(i + 4),
                            i + 1
                        ));
                    }
                    tokens.push(Token::Null);
                    i += 4;
                    continue;
                } else {
                    return Err(format!(
                        "Yanlış karakter kullanımı: {:?}, konum: {}",
                        chars.get(i + 4),
                        i + 1
                    ));
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
