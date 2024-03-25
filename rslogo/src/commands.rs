mod control;
mod factories;
mod procedure;
mod queries;
mod variables;
use crate::{
    ast::{BinaryExpression, CallExpression, Statement},
    heap::Heap,
    tokens::{IdentifierType, ValueType},
};

use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub enum Value {
    Identifier(IdentifierType),
    Literal(Literal),
    Statement(Box<CallExpression>),
    BinaryExpression(Box<BinaryExpression>),
}

pub type Literal = ValueType;

impl Literal {
    pub fn expect_int(&self) -> Option<i32> {
        match self {
            ValueType::Int(int) => Some(int.to_owned()),
            ValueType::Float(float) => {
                if ((*float as i32) as f32) == *float {
                    Some(float.to_owned() as i32)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn expect_float(&self) -> Option<f32> {
        match self {
            ValueType::Float(float) => Some(float.to_owned()),
            ValueType::Int(int) => Some(int.to_owned() as f32),
            _ => None,
        }
    }
    pub fn expect_bool(&self) -> Option<bool> {
        match self {
            ValueType::Bool(bool) => Some(bool.to_owned()),
            _ => None,
        }
    }
}

impl Value {
    pub fn expect_literal_r(&self, heap: &mut Heap) -> Option<Literal> {
        match self {
            Value::Identifier(identifier) => Some(
                heap.variable_value(
                    &identifier
                        .expect_variable()
                        .expect("Expect a variable or literal"),
                )
                .expect("variable does not exist"),
            ),
            Value::Literal(literal) => Some(literal.clone()),
            Value::Statement(statement) => Some(statement.call(heap).unwrap()),
            Value::BinaryExpression(binary) => Some(binary.calculate(heap)),
        }
    }
}

pub trait Callable {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal>;
}

pub trait Assignable {
    fn declare(&self, left: &str, right: &Value, heap: &mut Heap);
}

pub struct CallableFactory {}
pub struct AssignableFactory {}
pub struct Procedure {
    arguments: Vec<String>,
    body: Vec<Statement>,
}
