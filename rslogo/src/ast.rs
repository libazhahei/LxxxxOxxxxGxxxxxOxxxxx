use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use serde::Serialize;

use crate::{
    commands::{Literal, Procedure, Value},
    heap::Heap,
    tokens::{
        AssignmentType, CalculationOperation, CommandType, LogicalOperation, QueryType, TokenType,
    },
};
pub trait Executable {
    fn execute(&self, heap: &mut Heap);
}
impl Executable for Statement {
    fn execute(&self, heap: &mut Heap) {
        match self {
            Statement::CallExpression(call) => {
                call.call(heap);
            }
            Statement::AssignmentExpression(assign) => assign.execute(heap),
            Statement::If(if_statement) => if_statement.execute(heap),
            Statement::While(while_statement) => while_statement.execute(heap),
            Statement::ProcedureDeclaration(procedure_declaration) => {
                procedure_declaration.execute(heap)
            }
        };
    }
}
impl Executable for CallExpression {
    fn execute(&self, heap: &mut Heap) {
        self.call(heap);
    }
}

impl Executable for AssignmentExpression {
    fn execute(&self, heap: &mut Heap) {
        let assiganble = heap.declare_variable(&self.operation).clone();
        assiganble.declare(&self.left, &self.right, heap);
    }
}

impl Executable for IfStatement {
    fn execute(&self, heap: &mut Heap) {
        let test = self
            .test
            .expect_literal_r(heap)
            .expect("Literal is expected");
        if test.expect_bool().expect("Bool value is expected") {
            for statement in &self.consequent {
                statement.execute(heap);
            }
        }
    }
}

impl Executable for WhileStatement {
    fn execute(&self, heap: &mut Heap) {
        let mut test = self
            .test
            .expect_literal_r(heap)
            .expect("Literal is expected");
        while test.expect_bool().expect("Bool value is expected") {
            for statement in &self.body {
                statement.execute(heap);
            }
            test = self.test.expect_literal_r(heap).unwrap();
        }
    }
}

impl Executable for ProcedureDeclaration {
    fn execute(&self, heap: &mut Heap) {
        let body: Vec<Statement> = self.body.to_owned();
        let arguments = self.arguments.to_owned();
        heap.declare_procedure(self.name.clone(), Rc::new(Procedure::new(arguments, body)));
    }
}

#[derive(Clone, Serialize)]
pub enum Statement {
    CallExpression(CallExpression),
    AssignmentExpression(AssignmentExpression),
    If(IfStatement),
    While(WhileStatement),
    ProcedureDeclaration(ProcedureDeclaration),
}
impl Statement {
    pub fn expect_callexpression(&self) -> Option<CallExpression> {
        match self {
            Statement::CallExpression(callexpression) => Some(callexpression.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct AssignmentExpression {
    operation: AssignmentType,
    left: String,
    right: Value,
}
impl AssignmentExpression {
    pub fn new(operation: AssignmentType, left: String, right: Value) -> AssignmentExpression {
        AssignmentExpression {
            operation,
            left,
            right,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ProcedureDeclaration {
    name: String,
    arguments: Vec<String>,
    body: Vec<Statement>,
}

impl ProcedureDeclaration {
    pub fn new(name: String, arguments: Vec<String>, body: Vec<Statement>) -> ProcedureDeclaration {
        ProcedureDeclaration {
            name,
            arguments,
            body,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub enum BinaryOperation {
    Logic(LogicalOperation),
    Calculation(CalculationOperation),
}

#[derive(Clone, Serialize, Debug)]
pub struct BinaryExpression {
    operation: BinaryOperation,
    left: Value,
    right: Value,
}
impl BinaryExpression {
    pub fn new(operation: TokenType, left: Value, right: Value) -> BinaryExpression {
        let operation: BinaryOperation = match operation {
            TokenType::Logic(logic) => BinaryOperation::Logic(logic),
            TokenType::Calculation(calculation) => BinaryOperation::Calculation(calculation),
            _ => panic!("BinaryExpression can not be created from this token type"),
        };
        BinaryExpression {
            operation,
            left,
            right,
        }
    }

    pub fn calculate(&self, heap: &mut Heap) -> Literal {
        match &self.operation {
            BinaryOperation::Calculation(operation) => {
                let left = self
                    .left
                    .expect_literal_r(heap)
                    .expect("Literal is expected");
                let right = self
                    .right
                    .expect_literal_r(heap)
                    .expect("Literal is expected");
                match operation {
                    CalculationOperation::Plus => left.add(right),
                    CalculationOperation::Dash => left.sub(right),
                    CalculationOperation::Star => left.mul(right),
                    CalculationOperation::Slash => left.div(right),
                }
            }
            BinaryOperation::Logic(operation) => {
                let left = self
                    .left
                    .expect_literal_r(heap)
                    .expect("Literal is expected");
                let right = self
                    .right
                    .expect_literal_r(heap)
                    .expect("Literal is expected");

                match operation {
                    LogicalOperation::And => Literal::Bool(
                        left.expect_bool().expect("Bool value is expected")
                            && right.expect_bool().expect("Bool value is expected"),
                    ),
                    LogicalOperation::Or => Literal::Bool(
                        left.expect_bool().expect("Bool value is expected")
                            || right.expect_bool().expect("Bool value is expected"),
                    ),
                    LogicalOperation::Eq => Literal::Bool(left.eq(&right)),
                    LogicalOperation::Ne => Literal::Bool(left.ne(&right)),
                    LogicalOperation::Lt => Literal::Bool(left.lt(&right)),
                    LogicalOperation::Gt => Literal::Bool(left.gt(&right)),
                }
            }
        }
    }
}

#[derive(Clone, Serialize)]
pub struct WhileStatement {
    test: TestType,
    body: Vec<Statement>,
}

impl WhileStatement {
    pub fn new(test: TestType, body: Vec<Statement>) -> WhileStatement {
        WhileStatement { test, body }
    }
}
pub type TestType = Value;
#[derive(Clone, Serialize)]
pub struct IfStatement {
    test: TestType,
    consequent: Vec<Statement>,
}
impl IfStatement {
    pub fn new(test: TestType, consequent: Vec<Statement>) -> IfStatement {
        IfStatement { test, consequent }
    }
}
#[derive(Clone, Serialize, Debug)]
pub struct CallExpression {
    callee: Callee,
    arguments: Vec<Value>,
}
impl CallExpression {
    pub fn new(callee: Callee, arguments: Vec<Value>) -> CallExpression {
        CallExpression { callee, arguments }
    }
    pub fn call(&self, heap: &mut Heap) -> Option<Literal> {
        heap.execute_function(&self.callee, &self.arguments)
    }
}
#[derive(Clone, Eq, Hash, PartialEq, Serialize, Debug)]
pub enum Callee {
    Command(CommandType),
    Procedure(String),
    Query(QueryType),
}
// /*
impl AssignmentExpression {
    pub fn operation(&self) -> &AssignmentType {
        &self.operation
    }
    pub fn left(&self) -> &String {
        &self.left
    }
    pub fn right(&self) -> &Value {
        &self.right
    }
}
impl ProcedureDeclaration {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }
    pub fn body(&self) -> &Vec<Statement> {
        &self.body
    }
}
impl BinaryExpression {
    pub fn left(&self) -> &Value {
        &self.left
    }
    pub fn right(&self) -> &Value {
        &self.right
    }
    pub fn operation(&self) -> &BinaryOperation {
        &self.operation
    }
}
impl WhileStatement {
    pub fn test(&self) -> &TestType {
        &self.test
    }
    pub fn body(&self) -> &Vec<Statement> {
        &self.body
    }
}
impl IfStatement {
    pub fn test(&self) -> &TestType {
        &self.test
    }
    pub fn consequent(&self) -> &Vec<Statement> {
        &self.consequent
    }
}
impl CallExpression {
    pub fn callee(&self) -> &Callee {
        &self.callee
    }
    pub fn arguments(&self) -> &Vec<Value> {
        &self.arguments
    }
}
//  */
