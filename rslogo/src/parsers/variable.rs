use crate::{
    ast::{AssignmentExpression, Statement},
    commands::Value,
    tokens::{StatementParser, TokenType},
};

pub struct VariableAssignParser {}
impl StatementParser for VariableAssignParser {
    fn parse(&self, buf: &[TokenType]) -> Statement {
        VariableAssignParser::parse_variable_assign(buf)
    }
}

impl VariableAssignParser {
    fn parse_variable_assign(buf: &[TokenType]) -> Statement {
        let mut itr = buf.iter();
        if let Some(first) = itr.next() {
            let command = first
                .expect_assignment()
                .expect("Vairable Assign Parser accpect only MAKE and ADDASSIGN command");
            let left = itr
                .next()
                .expect("Missing left value")
                .expect_identifier()
                .expect("expect a left value")
                .expect_variable()
                .expect("Vairable name is used in persudure");
            let right = Value::parse_one(&mut itr);
            Statement::AssignmentExpression(AssignmentExpression::new(command, left, right))
        } else {
            panic!("")
        }
    }
}
