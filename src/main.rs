use std::println;

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
    let input = String::from("truetrue");
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
                let mut word = vec![character];
                let mut j = i + 1;
                let k = j + 3;
                while j <= k {
                    if j >= chars.len() && word.len() == 4 {
                        break;
                    } else if j >= chars.len() && word.len() != 4 {
                        return Err(format!(
                            "Yanlış karakter kullanımı veya bilinmeyen token: {}, konum: {}",
                            character,
                            i + 1
                        ));
                    }
                    let char = chars[j];
                    if j == (i + 1) && char == 'r' {
                        word.push(char);
                    } else if j == (i + 2) && char == 'u' {
                        word.push(char);
                    } else if j == (i + 3) && char == 'e' {
                        word.push(char);
                        if j == (chars.len() - 1) {
                            tokens.push(Token::True);
                            break;
                        }
                    } else if j == (i + 4)
                        && (char == '\n' || char == '\t' || char == '\r' || char == ' ')
                    {
                        tokens.push(Token::True);
                        break;
                    } else if j == (i + 4)
                        && (char == ','
                            || char == ':'
                            || char == '['
                            || char == ']'
                            || char == '{'
                            || char == '}')
                    {
                        tokens.push(Token::True);
                        break;
                    } else {
                        return Err(format!(
                            "Yanlış karakter kullanımı: {}, konum: {}",
                            chars[j],
                            j + 1
                        ));
                    };
                    j += 1;
                }
                i = k - 1;
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
