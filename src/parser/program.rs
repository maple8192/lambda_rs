use crate::parser::expr::{parse_expr, Expr};
use crate::parser::token_consumer::{expect_ident, expect_token};
use crate::token::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub main: Expr,
    pub functions: HashMap<String, Expr>,
}

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, String> {
    let mut tokens = &tokens[..];
    let mut functions = HashMap::new();
    let mut main = None;
    while !tokens.is_empty() {
        let func_name = expect_ident(&mut tokens)?;
        expect_token(&mut tokens, Token::Equal)?;
        let expr = parse_expr(&mut tokens, &functions)?;
        expect_token(&mut tokens, Token::Br)?;
        if functions.contains_key(&func_name) {
            return Err(format!("Already declared function {func_name}"));
        }
        if func_name == "main".to_string() {
            let None = main else {
                return Err("Already declared main".to_string());
            };
            main = Some(expr);
        } else {
            functions.insert(func_name, expr);
        }
    }

    let Some(main) = main else {
        return Err("Missing main".to_string());
    };
    Ok(Program { main, functions })
}
