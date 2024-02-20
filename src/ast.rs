use std::sync::Arc;

use derive_new::new;

use crate::tokens::{Literal, Symbol};
use crate::util::{CompoundExpression, DeclarationNode, ExpressionNode, StatementNode};

#[derive(Debug)]
pub enum ASTNode {
    Statement(StatementNode),
    Expression(ExpressionNode),
    Declaration(DeclarationNode),
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    Block(Vec<ASTNode>),
    Return(Expression),
}

#[derive(Debug, new)]
pub enum Expression {
    Variable(String),
    Literal(Literal),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    FunctionCall(FunctionCall),
    SizeOf(TypeOrIdentifier),
}

#[derive(Debug)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Debug, new)]
pub struct VariableDeclaration {
    pub name: String,
    pub ty: DataType,
}

#[derive(Debug, new)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: DataType,
    pub params: Vec<VariableDeclaration>,
    pub var_args: bool,
    pub body: Option<Vec<ASTNode>>,
}

#[derive(Debug, new)]
pub struct UnaryExpression {
    op: UnOp,
    right: Box<ExpressionNode>,
}

#[derive(Debug, new)]
pub struct BinaryExpression {
    left: Box<ExpressionNode>,
    op: BinOp,
    right: Box<ExpressionNode>,
}

#[derive(Debug, new)]
pub struct FunctionCall {
    pub name: String,
    pub args: CompoundExpression,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,

    LogicalAnd,
    LogicalOr,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    LeftShift,
    RightShift,

    Assign(AssignOp),
}

#[derive(Debug)]
pub enum AssignOp {
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug)]
pub enum UnOp {
    Negate,
    LogicalNot,
    BitwiseNot,
}

#[derive(Debug)]
pub enum TypeOrIdentifier {
    Type(DataType),
    Identifier(Arc<String>),
}

#[derive(Debug)]
pub enum DataType {
    Int,
    UnsignedInt,
    Float,
}

impl BinOp {
    pub fn is_assignment(&self) -> bool {
        matches!(self, BinOp::Assign(_))
    }

    pub fn precedence(&self) -> u8 {
        use BinOp::*;
        match self {
            Multiply | Divide | Modulo => 3,
            Add | Subtract => 2,
            Assign(_) => 1,
            _ => panic!("Invalid precedence for {:?}", self),
        }
    }
}

impl TryFrom<Symbol> for BinOp {
    type Error = ();

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        match value {
            Symbol::Plus => Ok(BinOp::Add),
            Symbol::Minus => Ok(BinOp::Subtract),
            Symbol::Star => Ok(BinOp::Multiply),
            Symbol::Slash => Ok(BinOp::Divide),
            Symbol::Modulo => Ok(BinOp::Modulo),

            Symbol::EqualEqual => Ok(BinOp::Equal),
            Symbol::BangEqual => Ok(BinOp::NotEqual),
            Symbol::GreaterThan => Ok(BinOp::GreaterThan),
            Symbol::GreaterThanEqual => Ok(BinOp::GreaterThanEqual),
            Symbol::LessThan => Ok(BinOp::LessThan),
            Symbol::LessThanEqual => Ok(BinOp::LessThanEqual),

            Symbol::Ampersand => Ok(BinOp::BitwiseAnd),
            Symbol::Pipe => Ok(BinOp::BitwiseOr),
            Symbol::Caret => Ok(BinOp::BitwiseXor),
            Symbol::LeftShift => Ok(BinOp::LeftShift),
            Symbol::RightShift => Ok(BinOp::RightShift),

            _ => Err(()),
        }
    }
}

impl TryFrom<Symbol> for UnOp {
    type Error = ();

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        match value {
            Symbol::Minus => Ok(UnOp::Negate),
            Symbol::Bang => Ok(UnOp::LogicalNot),
            Symbol::Tilde => Ok(UnOp::BitwiseNot),
            _ => Err(()),
        }
    }
}
