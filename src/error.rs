
#[derive(Debug)]
pub struct Error
{
    err: String,
    desc: String,
    trace_back: String,
    x: i32,
    y: i32
}

impl Error {
    pub fn none() -> Error
    {
        Error
        {
            err: "NONE".to_string(),
            desc: "NONE".to_string(),
            trace_back: "NONE".to_string(),
            x: -1,
            y: -1,
        }
    }
    pub fn lex_error() -> Error
    {
        Error
        {
            err: "Lexer".to_string(),
            desc: "NONE".to_string(),
            trace_back: "NONE".to_string(),
            x: -1,
            y: -1,
        }
    }
    pub fn bare_error(err: &str, desc: &str, x: i32, y: i32) -> Error
    {
        Error
        {
            err: err.to_string(),
            desc: desc.to_string(),
            trace_back: "NONE".to_string(),
            x,
            y
        }
    }
    pub fn expectation_error(desc: &str, x: i32, y: i32) -> Error
    {
        Error
        {
            err: "Expectation".to_string(),
            desc: desc.to_string(),
            trace_back: "NONE".to_string(),
            x,
            y
        }
    }
    pub fn redefine_error(desc: &str, x: i32, y: i32) -> Error
    {
        Error
        {
            err: "Redefinition".to_string(),
            desc: desc.to_string(),
            trace_back: "NONE".to_string(),
            x,
            y
        }
    }
    pub fn mismatched_type_error(desc: &str, x: i32, y: i32) -> Error
    {
        Error
        {
            err: "Mismatched types".to_string(),
            desc: desc.to_string(),
            trace_back: "NONE".to_string(),
            x,
            y
        }
    }
    pub fn undefined_error(desc: &str, x: i32, y: i32) -> Error
    {
        Error
        {
            err: "Undefined".to_string(),
            desc: desc.to_string(),
            trace_back: "NONE".to_string(),
            x,
            y
        }
    }
    pub fn is_none(&self) -> bool
    {
        self.err == "NONE"
    }
    pub fn show(&self)
    {
        println!("{} Error: {}, at char {}, line {}", self.err, self.desc, self.x, self.y);
    }
}
