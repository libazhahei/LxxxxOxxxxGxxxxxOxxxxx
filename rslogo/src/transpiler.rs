use std::fmt::Display;

use crate::{
    ast::{
        AssignmentExpression, BinaryExpression, BinaryOperation, CallExpression, Callee,
        IfStatement, ProcedureDeclaration, Statement, WhileStatement,
    },
    commands::Value,
    tokens::{
        AssignmentType, CalculationOperation, CommandType, LogicalOperation, QueryType, ValueType,
    },
};

pub trait Translater {
    fn to_python(&self, _num_tab: usize) -> String;
}

impl Translater for Statement {
    fn to_python(&self, _num_tab: usize) -> String {
        match self {
            Statement::CallExpression(call) => call.to_python(_num_tab),
            Statement::AssignmentExpression(assign) => assign.to_python(_num_tab),
            Statement::If(if_statement) => if_statement.to_python(_num_tab),
            Statement::While(while_statement) => while_statement.to_python(_num_tab),
            Statement::ProcedureDeclaration(procedure_declaration) => {
                procedure_declaration.to_python(_num_tab)
            }
        }
    }
}

impl Translater for CallExpression {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("{}{}(", "    ".repeat(_num_tab), self.callee()));
        let mut vec_list = Vec::new();
        for value in self.arguments() {
            vec_list.push(format!("{}", value));
        }
        buf.push_str(&format!("{})\n", vec_list.join(", ")));
        buf
    }
}
impl Translater for IfStatement {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("{}if {}:\n", "    ".repeat(_num_tab), self.test()));
        let mut vec_list = Vec::new();
        for statement in self.consequent() {
            vec_list.push(statement.to_python(_num_tab + 1));
        }
        buf.push_str(&format!("{}\n", vec_list.join("")));
        buf
    }
}
impl Translater for WhileStatement {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        buf.push_str(&format!(
            "{}while {}:\n",
            "    ".repeat(_num_tab),
            self.test()
        ));
        let mut vec_list = Vec::new();
        for statement in self.body() {
            vec_list.push(statement.to_python(_num_tab + 1));
        }
        buf.push_str(&format!("{}\n", vec_list.join("")));
        buf
    }
}

impl Translater for BinaryExpression {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        let left = self.left();
        let right = self.right();
        buf.push_str(&format!("({} {} {})", left, self.operation(), right));
        buf
    }
}
impl Translater for ProcedureDeclaration {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("def {}(", self.name()));
        let mut vec_list = Vec::new();
        for value in self.arguments() {
            vec_list.push(value.to_string());
        }
        buf.push_str(&format!("{}):\n", vec_list.join(", ")));
        for statement in self.body() {
            buf.push_str(&statement.to_python(_num_tab + 1).to_string());
        }
        buf.push('\n');
        buf
    }
}
impl Translater for AssignmentExpression {
    fn to_python(&self, _num_tab: usize) -> String {
        let mut buf = String::new();
        match self.operation() {
            AssignmentType::Make => buf.push_str(&format!(
                "{}{} {} {}\n",
                "    ".repeat(_num_tab),
                self.left(),
                self.operation(),
                self.right()
            )),
            AssignmentType::Addassign => buf.push_str(&format!(
                "{}{} {} {}\n",
                "    ".repeat(_num_tab),
                self.left(),
                self.operation(),
                self.right()
            )),
        };
        buf
    }
}
impl Display for AssignmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentType::Make => write!(f, "="),
            AssignmentType::Addassign => write!(f, "+="),
        }
    }
}
impl Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandType::Penup => write!(f, "penup"),
            CommandType::Forward => write!(f, "forward"),
            CommandType::Pendown => write!(f, "pendown"),
            CommandType::Back => write!(f, "back"),
            CommandType::Left => write!(f, "left"),
            CommandType::Right => write!(f, "right"),
            CommandType::Setheading => write!(f, "setheading"),
            CommandType::Setpencolor => write!(f, "setpencolor"),
            CommandType::Turn => write!(f, "turn"),
            CommandType::Setx => write!(f, "setx"),
            CommandType::Sety => write!(f, "sety"),
        }
    }
}
impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryType::Xcor => write!(f, "xcor"),
            QueryType::Ycor => write!(f, "ycor"),
            QueryType::Heading => write!(f, "heading"),
            QueryType::Color => write!(f, "pendown"),
        }
    }
}

impl Display for Callee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callee::Command(command) => write!(f, "{}", command),
            Callee::Query(query) => write!(f, "{}", query),
            Callee::Procedure(procedure) => write!(f, "{}", procedure),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(literal) => write!(f, "{}", literal),
            Value::Identifier(variable) => write!(f, "{}", variable.expect_variable().unwrap()),
            Value::Statement(stament) => write!(f, "{}", stament.to_python(0)),
            Value::BinaryExpression(binary) => write!(f, "{}", binary.to_python(0)),
        }
    }
}
impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Int(number) => write!(f, "{}", number),
            ValueType::Float(string) => write!(f, "{}", string),
            ValueType::Bool(boolean) => write!(f, "{}", boolean),
        }
    }
}
impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperation::Calculation(cal) => match cal {
                CalculationOperation::Plus => write!(f, "+"),
                CalculationOperation::Dash => write!(f, "-"),
                CalculationOperation::Star => write!(f, "*"),
                CalculationOperation::Slash => write!(f, "/"),
            },
            BinaryOperation::Logic(logic) => match logic {
                LogicalOperation::And => write!(f, "&&"),
                LogicalOperation::Or => write!(f, "||"),
                LogicalOperation::Eq => write!(f, "=="),
                LogicalOperation::Ne => write!(f, "!="),
                LogicalOperation::Lt => write!(f, "<"),
                LogicalOperation::Gt => write!(f, ">"),
            },
        }
    }
}
