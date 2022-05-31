use crate::error::Error;
use crate::lexer::{Lexer, Token, TokensWithLogAndError};
use crate::data_types::{YAL, VariableSingleValued, Py};

fn substring(str: String, start: i32, end: i32) ->  Option<String>
{
    if end <= start
    {
        return None;
    }
    let ss = (&str[(start as usize)..(end as usize)]).to_string();
    Option::from(ss)

}
fn startswith(string_big: String, string_small: &str) -> bool
{
    for (i, j) in string_big.chars().zip(string_small.chars())
    {
        if i != j
        {
            return false;
        }
    }
    true
}
fn pass()
{

}

pub struct Parser
{
    run: bool,
    to_parse: Vec<Token>,
    pub current_token: Token,
    lexer: Lexer,
    index: i32,
    yal: Vec<YAL>
}

impl Parser
{
    pub fn parser() -> Parser
    {
        Parser
        {
            run: true,
            to_parse: Vec::new(),
            current_token: Token{ value: "ERR".to_string(), token_type: "ERR".to_string(), x: -1, y: -1, is_data_type: true, is_keyword: false},
            lexer: Lexer::lexer(),
            index: -1,
            yal: Vec::new()
        }
    }
    fn next_token(&mut self) -> bool
    {
        self.index += 1;
        if self.to_parse.len() <= self.index as usize
        {
            return false;
        }
        self.current_token = self.to_parse[self.index as usize].clone();
        true
    }
    fn next_token_ignore_spaces(&mut self) -> bool
    {
        if !self.next_token()
        {
            return false
        };
        while self.current_token.is_space() {
            if !self.next_token()
            {
                return false
            }
        }
        true
    }
    fn get_token_ignore_spaces(&self) -> Option<&Token>
    {
        let mut ind =  1;
        let mut tok: &Token = &self.to_parse[(self.index + ind) as usize];
        while tok.is_space() {
            ind += 1;
            if self.to_parse.len() == (self.index + ind) as usize
            {
                return None;
            }
            tok = &self.to_parse[(self.index + ind) as usize];
        }
        if tok.is_space()
        {
            return None;
        }
        Some(tok)
    }
    fn has_token_after_spaces(&self) -> bool
    {
        let tok = self.get_token_ignore_spaces();
        let mut res = false;
        match tok
        {
            Some(_token) => res = true,
            None => pass(),
        };
        res
    }
    fn parse(&mut self) -> (Vec<YAL>, Error)
    {
        let types = vec!["var".to_string(), "int".to_string(), "str".to_string(),
                         "float".to_string(), "bool".to_string(), "list".to_string(),
                         "vec".to_string(), "map".to_string()];
        let ops = vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string()];

        let mut err = Error::none();
        /*
            var a > int = 12;
            var b < int = 12; # constant
            a = 12

        */

        while self.run {
            if self.next_token() {
                if self.current_token.token_type == "var_keyword"{
                    let mut is_const = false;

                    if !self.next_token_ignore_spaces() {
                        err = Error::expectation_error("Expected A variable Name", self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    else if &self.current_token.token_type != "identifier" {
                        err = Error::expectation_error(&format!("Expected A variable Name got '{}' instead", &self.current_token.value), self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    let var_name = self.current_token.clone();

                    if !self.next_token_ignore_spaces() {
                        err = Error::expectation_error("Expected a constant identifier", self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    else if &self.current_token.token_type != "greater than" && &self.current_token.token_type != "less than" {
                        err = Error::expectation_error(&format!("Expected a constant identifier got '{}' instead", &self.current_token.value), self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    else if &self.current_token.value == "<" {
                        is_const = true;
                    }

                    if !self.next_token_ignore_spaces() {
                        err = Error::expectation_error(&format!("Expected a variable type for to be defined '{}'", var_name.value), self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    else if !types.contains(&self.current_token.value) || self.current_token.token_type != "identifier" {
                        err = Error::expectation_error(&format!("Expected a variable type for to be defined '{}' got '{}' instead", var_name.value, &self.current_token.true_value()), self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    let var_type = self.current_token.clone();

                    if !self.next_token_ignore_spaces() {
                        err = Error::expectation_error("Expected an equals", self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }
                    else if &self.current_token.token_type != "equals" {
                        err = Error::expectation_error(&format!("Expected an equals got '{}' instead", &self.current_token.value), self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }

                    let mut var_values: Vec<Token> = Vec::new();
                    let mut expect_op = false;
                    while self.current_token.value != ";" {
                        if !self.run { break }

                        if self.next_token() {
                            if self.current_token.value == ";"{
                                break
                            }
                            else if self.current_token.is_data_type(){
                                if expect_op {
                                    err = Error::expectation_error(&format!("Expected an operation got '{}' instead", &self.current_token.value), self.current_token.x, self.current_token.y);
                                    self.run = false;
                                    break;
                                } else if self.current_token.token_type != var_type.value {
                                    err = Error::mismatched_type(&format!("Mismatched Type expected a '{}' got '{}' instead", &var_type.value, &self.current_token.token_type), self.current_token.x, self.current_token.y);
                                    self.run = false;
                                    break;
                                } else {
                                    var_values.push(self.current_token.clone());
                                    expect_op = true;
                                }
                            } else if ops.contains(&self.current_token.value) {
                                if !expect_op {
                                    err = Error::expectation_error(&format!("Expected a data got operation '{}' instead", &self.current_token.value), self.current_token.x, self.current_token.y);
                                    self.run = false;
                                    break;
                                } else {
                                    var_values.push(self.current_token.clone());
                                    expect_op = false;
                                }
                            } else if !self.current_token.is_space() {
                                println!("{:?}", self.current_token);
                                unimplemented!();
                            }
                        }
                        else {
                            err = Error::expectation_error("Expected the end of line", self.current_token.x, self.current_token.y);
                            self.run = false;
                            break;
                        }

                    }
                    if !self.run { break }

                    self.yal.push(YAL::VariableSingleValue(VariableSingleValued {
                        value: var_values,
                        data_type: var_type,
                        name: var_name,
                        is_const
                    }))

                }
                else if self.current_token.token_type == "comment" {
                    if startswith(self.current_token.value.clone(), "python:"){
                        let eval = substring(self.current_token.value.clone(), 7, self.current_token.value.len() as i32);
                        if !eval.is_none(){
                            self.yal.push(YAL::Py(Py { value: eval.unwrap().to_string() }));
                        } else {
                            err = Error::bare_error("Py to Yal",&format!("couldnt convert '{}'", self.current_token.value), self.current_token.x, self.current_token.y);
                            self.run = false;
                            break;
                        }
                    }
                }
            }
            else
            {
                self.run = false;
            }
        }
        (self.yal.clone(), err)

    }

    pub fn parse_text(&mut self, text: String) -> (Vec<YAL>, Error)
    {
        let response_from_lexer: TokensWithLogAndError = self.lexer.lex_text(text);
        if response_from_lexer.error.len() as u32 > 0
        {
            println!("Error: {:?}", &response_from_lexer.error);
            return (Vec::new(), Error::lex_error());
        }
        self.to_parse = response_from_lexer.tokens;
        println!();
        println!("------------------");
        println!("to parse: {:?}", self.to_parse);
        println!("------------------");
        println!();
        self.parse()
    }
}
