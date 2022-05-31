
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Token {
    pub value: String,
    pub token_type: String,
    pub x: i32,
    pub y: i32
}

impl Token {
    pub fn is_space(&self) -> bool {
        let a = [" ", "\t", "\n"];
        for i in a {
           if i == self.value
           {
               return true;
           }
        }
        false
    }
    pub fn true_value(&self) -> String
    {
        let mut value = "".to_string();
        if self.token_type != "string"
        {

            value += &self.value;
        }
        else
        {
            value += "\"";
            value += &self.value;
            value += "\"";
        }
        value
    }
    pub fn set_origin(&mut self, x: i32, y :i32) {
        self.x = x;
        self.y = y;
    }
    pub fn is_data_type(&self) -> bool
    {
        vec!["int".to_string(), "str".to_string(), "bool".to_string(), "float".to_string(), "list".to_string()].contains(&self.token_type)
    }
    pub fn to_string(&self) -> String {
        format!("Token {} '{}' at char {}, line {}", self.token_type, self.value, self.x, self.y)
    }
}

pub struct Lexer {
    defined_tokens: HashMap<String, Token>,
    text_to_lex: Vec<char>,
    index: i32,
    run: bool,
    current_char: char,
    x: i32,
    y: i32,
    tok_start_x: i32,
    tok_start_y: i32,
    the_nums: Vec<char>,
    log: Vec<String>,
    error: Vec<String>,
}

#[derive(Debug)]
pub struct TokensWithLogAndError
{
    pub tokens: Vec<Token>,
    pub log: Vec<String>,
    pub error: Vec<String>,
}


impl Lexer {
    pub fn lexer() -> Lexer {
        let mut lexer = Lexer{defined_tokens: HashMap::new(), text_to_lex: [].to_vec(), index: -1, run: true, 
            current_char: ' ', x: 0, y: 1, the_nums: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
            log: Vec::new(), error: Vec::new(), tok_start_x: 0,
            tok_start_y: 0};

        lexer.defined_tokens.insert(String::from(" "), Token{value: String::from(" "), token_type: String::from("space"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("+"), Token{value: String::from("+"), token_type: String::from("plus"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("-"), Token{value: String::from("-"), token_type: String::from("dash"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("/"), Token{value: String::from("/"), token_type: String::from("slash"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("*"), Token{value: String::from("*"), token_type: String::from("asterisk"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("("), Token{value: String::from("("), token_type: String::from("paren lft"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from(")"), Token{value: String::from(")"), token_type: String::from("paren rht"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("{"), Token{value: String::from("{"), token_type: String::from("curbrac lft"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("}"), Token{value: String::from("}"), token_type: String::from("curbrac rht"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("["), Token{value: String::from("["), token_type: String::from("brac lft"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("]"), Token{value: String::from("]"), token_type: String::from("brac rht"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("<"), Token{value: String::from("<"), token_type: String::from("less than"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from(">"), Token{value: String::from(">"), token_type: String::from("greater than"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("&"), Token{value: String::from("&"), token_type: String::from("and"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("%"), Token{value: String::from("%"), token_type: String::from("percent"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("^"), Token{value: String::from("^"), token_type: String::from("caret"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("'"), Token{value: String::from("'"), token_type: String::from("single quote"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("\""), Token{value: String::from("\""), token_type: String::from("dub quote"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("."), Token{value: String::from("."), token_type: String::from("period"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from(";"), Token{value: String::from(";"), token_type: String::from("semi colon"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from(":"), Token{value: String::from(":"), token_type: String::from("colon"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("$"), Token{value: String::from("$"), token_type: String::from("dollar sign"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("="), Token{value: String::from("="), token_type: String::from("equals"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("@"), Token{value: String::from("@"), token_type: String::from("at"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("!"), Token{value: String::from("!"), token_type: String::from("excl mark"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("#"), Token{value: String::from("#"), token_type: String::from("hash tag"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from(","), Token{value: String::from(","), token_type: String::from("comma"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("\t"), Token{value: String::from("\t"), token_type: String::from("indent"), x: -1, y: -1});
        lexer.defined_tokens.insert(String::from("\n"), Token{value: String::from("\n"), token_type: String::from("new line"), x: -1, y: -1});

        lexer
    }
    fn newline(&mut self) {
        self.y += 1;
        self.x = 0;
    }

    fn next_char(&mut self) -> bool {
        self.x += 1;
        self.index += 1;
        if self.index >= self.text_to_lex.len() as i32 {
            return false;
        }
        self.current_char = self.text_to_lex[self.index as usize];
        true

    }
    pub fn lex_text(&mut self, text_to_be_lexed: String) -> TokensWithLogAndError {
        self.text_to_lex = text_to_be_lexed.chars().collect();
        self.lex()
    }

    fn log(&mut self, to_log: String) {
        self.log.push(to_log + &*format!(" at line {}, char {}", self.y, self.x));
    }

    fn error(&mut self, to_error: String) {
        self.error.push(to_error + &*format!(" at line {}, char {}", self.y, self.x));
    }
    fn start_origin(&mut self)
    {
        self.tok_start_x = self.x;
        self.tok_start_y = self.y;
    }

    fn lex(&mut self) -> TokensWithLogAndError {
        let mut current_tokens: Vec<Token> = Vec::new();
        let mut comment_on = false;
        let mut str_on = false;
        let mut unk_on = false;
        let mut number_on = false;
        let mut number_type = String::from("int");
        let mut non_single_char = String::from("");

        while self.run {
            if self.next_char() {
                if self.current_char == '/' && !str_on
                {
                    if comment_on
                    {
                        comment_on = false;
                        current_tokens.push(Token{
                            value: non_single_char.clone(),
                            token_type: String::from("comment"),
                            x: self.tok_start_x, y: self.tok_start_y
                        });
                        non_single_char = String::from("");
                    }
                    else {
                        comment_on = true;
                        self.start_origin()
                    }
                }
                else if self.current_char == '"' && !comment_on
                {
                    if str_on
                    {
                        str_on = false;
                        current_tokens.push(Token{
                            value: non_single_char.clone(),
                            token_type: String::from("string"),
                            x: self.tok_start_x, y: self.tok_start_y
                        });
                        non_single_char = String::from("");
                    }
                    else { str_on = true; self.start_origin()}
                }
                else if str_on || comment_on
                {
                    non_single_char += &String::from(self.current_char);
                }
                else if self.current_char == '.' && number_on {
                    if number_type == "float"
                    {
                        self.error(String::from("Syntax Error, mo fo"));
                        self.run = false;
                    }
                    non_single_char += &String::from(self.current_char);
                    number_type = String::from("float");
                    self.log(number_type.to_string());
                }
                else if self.defined_tokens.contains_key(&String::from(self.current_char))
                {
                    if unk_on
                    {
                        unk_on = false;
                        current_tokens.push(Token{
                            value: non_single_char.clone(),
                            token_type: String::from("unknown"),
                            x: self.tok_start_x, y: self.tok_start_y
                        });
                        non_single_char = String::from("");
                    }
                    else if number_on
                    {
                        number_on = false;
                        current_tokens.push(Token{
                            value: non_single_char.clone(),
                            token_type: number_type.clone(),
                            x: self.tok_start_x, y: self.tok_start_y
                        });
                        non_single_char = String::from("");
                    }
                    let mut new_tok: Token = self.defined_tokens.get(&String::from(self.current_char)).unwrap().clone();
                    new_tok.y = self.y;
                    new_tok.x = self.x;
                    current_tokens.push(new_tok);
                    if self.current_char == '\n'
                    {
                        self.newline();
                    }
                }
                else if self.the_nums.contains(&self.current_char)
                {
                    if !number_on
                    {

                        if !non_single_char.is_empty()
                        {
                            self.error(format!("Error, non_single_char has a value {}", non_single_char));
                            self.run = false;
                        }
                        self.start_origin();
                        number_on = true;
                        non_single_char += &String::from(self.current_char);

                    }
                    else if number_on { non_single_char += &String::from(self.current_char); }
                    else
                    {
                       self.error(format!("huh? {}", self.current_char));
                       self.run = false;
                    }
                }
                else
                {
                    if unk_on { non_single_char += &String::from(self.current_char); }
                    else
                    {
                        self.start_origin();
                        unk_on = true;
                        non_single_char += &String::from(self.current_char);
                    }
                }
            }
            else
            {
                if unk_on
                {
                    unk_on = false;
                    current_tokens.push(Token{
                        value: non_single_char.clone(),
                        token_type: String::from("unknown"),
                        x: self.tok_start_x, y: self.tok_start_y
                    });
                    non_single_char = String::from("");
                }
                else if number_on
                {
                    number_on = false;
                    current_tokens.push(Token{
                        value: non_single_char.clone(),
                        token_type: number_type.clone(),
                        x: self.tok_start_x, y: self.tok_start_y
                    });
                    non_single_char = String::from("");
                }
                self.run = false;
            }
        }
        TokensWithLogAndError {tokens: current_tokens, log: self.log.clone(), error: self.error.clone()}
    }
}

/*
fn main() {
    let data = fs::read_to_string("./files/read.txt").expect("Unable to read file");
    let mut lexer = Lexer::lexer();
    let mut a = 0;
    for token in &lexer.lex_text(String::from(data)).tokens {
        a += 1;
        println!("{}. {:?}", a, token);
    }
}
 */