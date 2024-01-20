use crate::token::token::Token;

pub fn consume_token(tokens: &mut &[Token], expected: Token) -> bool {
    let Some(first) = tokens.first() else {
        return false;
    };
    return if *first == expected {
        *tokens = &tokens[1..];
        true
    } else {
        false
    };
}

pub fn expect_token(tokens: &mut &[Token], expected: Token) -> Result<(), String> {
    let Some(first) = tokens.first() else {
        return Err("End of token".to_string());
    };
    return if *first == expected {
        *tokens = &tokens[1..];
        Ok(())
    } else {
        Err(format!("Unexpected token {:?}", first))
    };
}

pub fn check_token(tokens: &mut &[Token], expected: Token) -> bool {
    let Some(first) = tokens.first() else {
        return false;
    };
    *first == expected
}

pub fn expect_ident(tokens: &mut &[Token]) -> Result<String, String> {
    let Some(first) = tokens.first() else {
        return Err("End of token".to_string());
    };
    return if let Token::Ident(ident) = first {
        *tokens = &tokens[1..];
        Ok(ident.clone())
    } else {
        Err(format!("Unexpected token {:?}", first))
    };
}
