use crate::parser::token_consumer::{check_token, consume_token, expect_ident, expect_token};
use crate::token::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Lambda(String, isize, Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
    FuncCall(String),
    Variable(String, isize),
    Ident(String),
}

pub fn parse_expr(
    tokens: &mut &[Token],
    functions: &HashMap<String, Expr>,
) -> Result<Expr, String> {
    let mut local_vars = Vec::new();
    let mut id = 0;
    return parse_lambda(tokens, functions, &mut local_vars, &mut id);
}

pub fn parse_lambda(
    tokens: &mut &[Token],
    functions: &HashMap<String, Expr>,
    local_vars: &mut Vec<(String, isize)>,
    id: &mut isize,
) -> Result<Expr, String> {
    if consume_token(tokens, Token::Lambda) {
        let var_name = expect_ident(tokens)?;
        if functions.contains_key(&var_name) {
            return Err(format!("Already declared as function {var_name}"));
        }
        if local_vars.iter().any(|x| x.0 == var_name) {
            return Err(format!("Already declared variable {var_name}"));
        }

        let var_id = *id;
        local_vars.push((var_name.clone(), var_id));
        *id += 1;

        expect_token(tokens, Token::Dot)?;

        let expr = parse_lambda(tokens, functions, local_vars, id)?;
        local_vars.pop().unwrap();

        return Ok(Expr::Lambda(var_name, var_id, Box::new(expr)));
    }

    parse_application(tokens, functions, local_vars, id)
}

pub fn parse_application(
    tokens: &mut &[Token],
    functions: &HashMap<String, Expr>,
    local_vars: &mut Vec<(String, isize)>,
    id: &mut isize,
) -> Result<Expr, String> {
    let mut expr = parse_primary(tokens, functions, local_vars, id)?;

    loop {
        if check_token(tokens, Token::Br) || check_token(tokens, Token::CloseParen) {
            return Ok(expr);
        }

        let rhs = parse_primary(tokens, functions, local_vars, id)?;
        expr = Expr::Application(Box::new(expr), Box::new(rhs));
    }
}

pub fn parse_primary(
    tokens: &mut &[Token],
    functions: &HashMap<String, Expr>,
    local_vars: &mut Vec<(String, isize)>,
    id: &mut isize,
) -> Result<Expr, String> {
    if consume_token(tokens, Token::OpenParen) {
        let expr = parse_lambda(tokens, functions, local_vars, id)?;
        expect_token(tokens, Token::CloseParen)?;
        return Ok(expr);
    }

    let ident = expect_ident(tokens)?;
    if functions.contains_key(&ident) {
        return Ok(Expr::FuncCall(ident));
    }
    if let Some(var) = local_vars.iter().rfind(|x| x.0 == ident) {
        return Ok(Expr::Variable(var.0.clone(), var.1));
    }
    Ok(Expr::Ident(ident))
}
