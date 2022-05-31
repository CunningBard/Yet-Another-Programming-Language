use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct VariableSingleValued
{
    pub value: Vec<Token>,
    pub data_type: Token,
    pub name: Token,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct Py
{
    pub value: String
}

#[derive(Debug, Clone)]
pub enum YAL
{
    VariableSingleValue(VariableSingleValued),
    Py(Py)
}