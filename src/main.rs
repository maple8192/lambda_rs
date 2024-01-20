use crate::calculator::calculator::run;
use crate::io::reader::read_file;
use crate::parser::program::parse_program;
use crate::token::tokenizer::tokenize;
use std::error::Error;

mod calculator;
mod io;
mod parser;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let code = read_file("test.lmd")?;
    let tokens = tokenize(code)?;
    let program = parse_program(tokens)?;
    run(program);
    Ok(())
}
