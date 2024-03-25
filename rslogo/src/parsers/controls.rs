use crate::{
    ast::{CallExpression, Callee, Statement},
    commands::Value,
    tokens::{CommandType, StatementParser, TokenType},
};

pub struct ControlsParser {}
impl StatementParser for ControlsParser {
    fn parse(&self, buf: &[TokenType]) -> Statement {
        ControlsParser::parse_controls(buf)
    }
}
impl ControlsParser {
    fn parse_controls(buf: &[TokenType]) -> Statement {
        let mut itr = buf.iter();
        let first = itr.next().expect("buf is empty");
        let command = first
            .expect_command()
            .expect("COntrols Parser accpect only command");
        let mut args = Vec::new();
        // println!("{}", serde_json::to_string_pretty(buf).unwrap());
        while itr.len() > 0 {
            let arg_value = Value::parse_one(&mut itr);
            // arg.to_value().expect("It is not epxected data type");
            args.push(arg_value);
        }
        if ControlsParser::args_check(&command, &args) {
            let callee = Callee::Command(command);
            Statement::CallExpression(CallExpression::new(callee, args))
        } else {
            panic!("Error number of args")
        }
    }
    fn args_check(command: &CommandType, args: &Vec<Value>) -> bool {
        // match (command, args.len()) {
        //     (CommandType::Penup, 0) => true,
        //     (CommandType::Pendown, 0) => true,
        //     (CommandType::Forward, 1) => true,
        //     (CommandType::Back, 1) => true,
        //     (CommandType::Left, 1) => true,
        //     (CommandType::Right, 1) => true,
        //     (CommandType::Setpencolor, 1) => true,
        //     (CommandType::Turn, 1) => true,
        //     (CommandType::Setheading, 1) => true,
        //     (CommandType::Setx, 1) => true,
        //     (CommandType::Sety, 1) => true,
        //     _ => false,
        // }
        matches!(
            (command, args.len()),
            (CommandType::Penup, 0)
                | (CommandType::Pendown, 0)
                | (CommandType::Forward, 1)
                | (CommandType::Back, 1)
                | (CommandType::Left, 1)
                | (CommandType::Right, 1)
                | (CommandType::Setpencolor, 1)
                | (CommandType::Turn, 1)
                | (CommandType::Setheading, 1)
                | (CommandType::Setx, 1)
                | (CommandType::Sety, 1)
        )
    }
}
