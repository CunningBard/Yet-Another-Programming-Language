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
fn startswith(string_big: String, string_small: String) -> bool
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
            current_token: Token{ value: "ERR".to_string(), token_type: "ERR".to_string(), x: -1, y: -1},
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
        self.next_token();
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
        let defined = vec!["var".to_string(), "int".to_string(), "string".to_string(),
                           "float".to_string(), "bool".to_string(), "list".to_string(),
                           "vec".to_string(), "map".to_string()];
        let types = vec!["var".to_string(), "int".to_string(), "string".to_string(),
                         "float".to_string(), "bool".to_string(), "list".to_string(),
                         "vec".to_string(), "map".to_string()];
        let mut err = Error::none();
        /*
            var a > int = 12;
            var b < int = 12; # constant
            a = 12

        */

        while self.run {
            if self.next_token()
            {
                if self.current_token.value == "var" && self.current_token.token_type == "unknown"
                {
                    // var name
                    if !self.has_token_after_spaces()
                    {
                        err = Error::expectation_error("Expected Variable name".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    self.next_token_ignore_spaces();
                    if self.current_token.is_data_type() || defined.contains(&self.current_token.value)
                    {
                        err = Error::redefine_error(format!("Cannot redefine '{}'", self.current_token.value),
                                                    self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    let var_name = self.current_token.clone();
                    let mut is_const = false;

                    // const  identifier
                    self.next_token_ignore_spaces();
                    if self.current_token.token_type == "string" ||
                        self.current_token.token_type == "unknown"
                    {
                        err = Error::expectation_error("Expected constant identifier".to_string(),
                                                        self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    else if self.current_token.value == "<"
                    {
                        is_const = true;
                    }
                    else if self.current_token.value != ">"
                    {
                        err = Error::expectation_error("Expected constant identifier".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }

                    // var type
                    if !self.has_token_after_spaces()
                    {
                        err = Error::expectation_error("Expected a type for the variable".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    self.next_token_ignore_spaces();
                    if self.current_token.token_type != "unknown" || !types.contains(&self.current_token.value)
                    {
                        err = Error::expectation_error("Expected a type for the variable".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    let var_type = self.current_token.clone();

                    if !self.has_token_after_spaces()
                    {
                        err = Error::expectation_error("Expected value assignment indicator ' = ' ".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    self.next_token_ignore_spaces();
                    if self.current_token.token_type != "equals"
                    {
                        err = Error::expectation_error("Expected value assignment indicator ' = ' ".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break
                    }
                    // = checking thing

                    self.next_token();
                    let mut var_value: Vec<Token> =  Vec::new();
                    let mut expect_op = false;
                    while self.current_token.value != ";" {
                        var_value.push(self.current_token.clone());
                        let a = self.next_token();
                        if !a
                        {
                            err = Error::expectation_error("Expected end of line".to_string(),
                                                           self.current_token.x, self.current_token.y);
                            self.run = false;
                            break;
                        }
                        else if self.current_token.value == ";"
                        {
                            break;
                        }
                        else if self.current_token.is_space() { }
                        else if ["+".to_string(), "/".to_string(), "-".to_string(), "*".to_string(),].contains(&self.current_token.value)
                        {
                            if !expect_op
                            {
                                err = Error::expectation_error("Expected a data type instead of a operator".to_string(),
                                                               self.current_token.x, self.current_token.y);
                                self.run = false;
                                break;
                            }
                            expect_op = false;
                        }
                        else if self.current_token.is_data_type()
                        {
                            if expect_op
                            {
                                err = Error::expectation_error("Expected a data type instead of a operator".to_string(),
                                                               self.current_token.x, self.current_token.y);
                                self.run = false;
                                break;
                            }
                            expect_op = true;
                        }
                        else if !self.current_token.is_data_type()
                        {
                            println!("{:?}", self.current_token);
                            err = Error::expectation_error("Expected a data type ".to_string(),
                                                           self.current_token.x, self.current_token.y);
                            self.run = false;
                            break;
                        }
                        else
                        {
                            println!(" >> {:?}", self.current_token);
                        }
                    }
                    if !self.run { break; }
                    else if var_value.is_empty()
                    {
                        err = Error::expectation_error("Expected values for assigning".to_string(),
                                                       self.current_token.x, self.current_token.y);
                        self.run = false;
                        break;
                    }

                    self.yal.push(YAL::VariableSingleValue(VariableSingleValued
                    {
                        value: var_value,
                        name: var_name,
                        data_type: var_type,
                        is_const,
                    }));
                }
                else if startswith(self.current_token.value.to_string(),
                                   "python:".to_string()) &&
                    self.current_token.token_type == "comment"
                {
                    let b = substring(self.current_token.value.to_string(),
                                      7, self.current_token.value.len() as i32).unwrap();
                    self.yal.push(YAL::Py(Py{value: b}))
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
        // println!("{:?}", self.to_parse);
        self.parse()
    }
}
