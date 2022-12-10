use crate::lexer::TokenType;

pub struct SourceUnit(pub Vec<SourceUnitPart>);

pub enum SourceUnitPart {
    Statement(Statement),
}

pub enum Statement {
    Declaration(Expression),
    Assignment(Expression, Expression),
    Write(Expression),
}

pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),

    Integer(TokenType),
    Symbol(TokenType),

    StringLiteral(TokenType),
    InputString,
    InputNumber,
}
