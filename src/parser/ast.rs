use std::fmt;

use crate::parser::token::TokenType;
use crate::parser::form::Form;
use crate::parser::position::Position;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Identifier(Identifier)
}

impl Expression {
    pub fn as_str(&self) -> String {
        match &self {
            Expression::BinaryOperation(b) => format!("({} {} {})", b.loperand.as_str(), b.operator, b.roperand.as_str()),
            Expression::UnaryOperation(u) => format!("({} {})", u.operator, u.operand.as_str()),
            Expression::Literal(l) => format!("{}", l),
            Expression::Identifier(i) => format!("{}", i.name)
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    form: Form,
    position: Position
}

impl Literal {
    pub fn value(&self) -> String{
        match &self.form {
            Form::Integer(i) => i.to_string()
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub loperand: Box<Expression>,
    pub operator: TokenType,
    pub roperand: Box<Expression>
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: TokenType,
    pub operand: Box<Expression>
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
    position: Position
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    Body(Body)
}

impl Statement {
    pub fn as_str(&self) -> String {
        match &self {
            Statement::Assignment(asg) => format!("Assignment[{}, {}]", asg.identifier, asg.value.as_str()),
            Statement::Body(bdy) => format!("Body[{}]", bdy)
        }
    }
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
    pub value: Box<Expression>,
    pub position: Position
}

#[derive(Debug)]
pub struct Body {
    code: Vec<AST>
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for code in &self.code {
            write!(f, "{}", code)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
pub enum AST {
    Statement(Statement),
    Expression(Expression)
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AST::Expression(expr) => write!(f, "{}", expr.as_str()),
            AST::Statement(stmt) => write!(f, "{}", stmt.as_str())
        }
    }
}