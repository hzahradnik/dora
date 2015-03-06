use std::collections::HashMap;

use data_type::DataType;
use lexer::position::Position;

#[derive(PartialEq,Eq,Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(PartialEq,Eq,Debug)]
pub struct Function {
    pub name: String,
    pub position: Position,
    pub params: Vec<usize>,
    pub vars: Vec<LocalVar>,
    pub block: Box<Statement>,
}

impl Function {
    pub fn new(name: String, pos: Position) -> Function {
        Function {
            name: name,
            position: pos,
            params: vec![],
            vars: vec![],
            block: Statement::new( Position::new(1,1), StatementType::Nop),
        }
    }

    pub fn exists(&self, var: &str) -> bool {
        self.vars.iter().any(|x| x.name.as_slice() == var)
    }

    pub fn add_param(&mut self, var: LocalVar) {
        let ind = self.vars.len();
        self.vars.push(var);
        self.params.push(ind);
    }
}

#[derive(PartialEq,Eq,Debug)]
pub struct LocalVar {
    pub name: String,
    pub position: Position,
    pub data_type: DataType,
}

impl LocalVar {
    pub fn new(name: String, data_type: DataType, position: Position) -> LocalVar {
        LocalVar { name: name, data_type: data_type, position: position }
    }
}

#[derive(PartialEq,Eq,Debug)]
pub struct Statement {
    pub position: Position,
    pub stmt: StatementType
}

impl Statement {
    pub fn new(pos: Position, stmt: StatementType) -> Box<Statement> {
        box Statement { position: pos, stmt: stmt }
    }

    pub fn expr(pos: Position, expr: Box<Expr>) -> Box<Statement> {
        Statement::new(pos, StatementType::Expr(expr))
    }

    pub fn empty_block(pos: Position) -> Box<Statement> {
        Statement::new(pos, StatementType::Block(vec![]))
    }

    pub fn block(pos: Position, stmt: Box<Statement>) -> Box<Statement> {
        Statement::new(pos, StatementType::Block(vec![stmt]))
    }
}

#[derive(PartialEq,Eq,Debug)]
pub enum StatementType {
    Var(String,DataType,Box<Expr>),
    While(Box<Expr>,Box<Statement>),
    Loop(Box<Statement>),
    If(Box<Expr>,Box<Statement>,Option<Box<Statement>>),
    Expr(Box<Expr>),
    Block(Vec<Box<Statement>>),
    Break,
    Continue,
    Return(Box<Expr>),
    Nop,
}

#[derive(PartialEq,Eq,Debug)]
pub enum UnOp {
    Plus,
    Neg,
}

#[derive(PartialEq,Eq,Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(PartialEq,Eq,Debug)]
pub struct Expr {
    pub position: Position,
    pub data_type: DataType,
    pub expr: ExprType,
}

impl Expr {
    pub fn new(pos: Position, data_type: DataType, expr: ExprType) -> Box<Expr> {
        box Expr { position: pos, data_type: data_type, expr: expr }
    }

    pub fn lit_int(pos: Position, value: i64) -> Box<Expr> {
        Expr::new(pos, DataType::Int, ExprType::LitInt(value))
    }

    pub fn lit_str(pos: Position, value: String) -> Box<Expr> {
        Expr::new(pos, DataType::Str, ExprType::LitStr(value))
    }

    pub fn ident(pos: Position, value: &'static str) -> Box<Expr> {
        Expr::new(pos, DataType::Int, ExprType::Ident(value.to_string()))
    }
}

#[derive(PartialEq,Eq,Debug)]
pub enum ExprType {
    Un(UnOp,Box<Expr>),
    Bin(BinOp,Box<Expr>,Box<Expr>),
    LitInt(i64),
    LitStr(String),
    LitTrue,
    LitFalse,
    Ident(String),
    Assign(Box<Expr>,Box<Expr>),
    Call(String,Vec<Box<Expr>>),
}

