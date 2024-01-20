use crate::token::token::Token;

pub fn tokenize(code: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut i = 0;
    while i < code.len() {
        match code.chars().nth(i).unwrap() {
            ' ' | '\r' => i += 1,
            '\n' => {
                i += 1;
                let Some(last) = tokens.last() else { continue };
                let Token::Br = last else {
                    tokens.push(Token::Br);
                    continue;
                };
            }
            '\\' => {
                i += 1;
                tokens.push(Token::Lambda)
            }
            '.' => {
                i += 1;
                tokens.push(Token::Dot)
            }
            '=' => {
                i += 1;
                tokens.push(Token::Equal)
            }
            '(' => {
                i += 1;
                tokens.push(Token::OpenParen)
            }
            ')' => {
                i += 1;
                tokens.push(Token::CloseParen)
            }
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'' => {
                let mut j = i;
                while j < code.len() {
                    let ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'') =
                        code.chars().nth(j).unwrap()
                    else {
                        break;
                    };
                    j += 1;
                }
                tokens.push(Token::Ident(
                    code.chars().collect::<Vec<char>>()[i..j].iter().collect(),
                ));
                i = j;
            }
            _ => return Err(format!("Unknown token {}", code.chars().nth(i).unwrap())),
        }
    }

    let Some(last) = tokens.last() else {
        return Ok(vec![Token::Br]);
    };
    let Token::Br = last else {
        tokens.push(Token::Br);
        return Ok(tokens);
    };
    Ok(tokens)
}
