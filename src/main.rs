extern crate core;

use std::fs;
mod compiler;
mod parser;
mod error;
mod lexer;
mod data_types;
use std::env;



fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut path ="in/main.yapl";
    if args.len() > 1{
        path = args.get(1).unwrap();
    }
    let mut parser = parser::Parser::parser();
    let file = fs::read_to_string(path);
    if file.is_ok()
    {
        let read = file.unwrap();
        println!("{}", read);
        /*
        let mut lexer = lexer::Lexer::lexer();
        let b = lexer.lex_text(read.clone());
        if !b.error.is_empty()
        {
            print!("{:?}", b.error);
        }
        else {
            for token in b.tokens
            {
                println!("{:?}", token);
            }
        }
        */

        let parser_result = parser.parse_text(read);
        if parser_result.1.is_none()
        {
            // println!("{:?}", parser_result.0);
            let compiler_result = compiler::compyle(parser_result.0);
            println!("{}", compiler_result);
            let _res = fs::write("out/out.py", compiler_result);
        }
        else
        {
            parser_result.1.show()
        }
    } else {
        println!("file couldnt be ran, '{}' was not found", path)
    }
}