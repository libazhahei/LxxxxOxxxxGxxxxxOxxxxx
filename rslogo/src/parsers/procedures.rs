use crate::{
    ast::{CallExpression, Callee, ProcedureDeclaration, Statement},
    commands::Value,
    tokens::{ASTParser, KeywordType, StatementParser, TokenType},
};

enum ProcedureParserState {
    Define,
    Call,
}

pub struct ProcedureParser {}

impl StatementParser for ProcedureParser {
    fn parse(&self, buf: &[TokenType]) -> Statement {
        // VariableAssignParser::parse_variable_assign(buf)
        // println!("Procedure Parser : {:?}", buf);
        match syntax_check(buf) {
            ProcedureParserState::Define => parse_procedure(buf),
            ProcedureParserState::Call => parse_call(buf),
        }
    }
}

fn parse_procedure(buf: &[TokenType]) -> Statement {
    let mut itr = buf.iter().skip(1);
    let name = itr
        .next()
        .expect("Procedure Parser accept only one token")
        .expect_identifier()
        .expect("Identifier is expected")
        .expect_procedure()
        .expect("Procedure name is used as variable name");
    let args: Vec<String> = itr
        .take_while(|token| !token.is_space())
        .map(|token| {
            token
                .expect_identifier()
                .unwrap()
                .expect_variable()
                .unwrap()
        })
        .collect();
    let mut body: Vec<TokenType> = buf
        .iter()
        .skip_while(|token| !token.is_space())
        .skip(1)
        .cloned()
        .collect();
    body.pop();
    let body = ASTParser::parse_from_tokens(&body);
    Statement::ProcedureDeclaration(ProcedureDeclaration::new(name, args, body))
}

fn parse_call(buf: &[TokenType]) -> Statement {
    let mut itr = buf.iter();
    let name = itr
        .next()
        .expect("Procedure Parser accept only one token")
        .expect_identifier()
        .expect("Identifier is expected")
        .expect_procedure()
        .expect("Procedure name is used as variable name");
    let mut args = Vec::new();
    while itr.len() > 0 {
        let arg_value = Value::parse_one(&mut itr);
        args.push(arg_value);
    }
    let callee = Callee::Procedure(name);
    Statement::CallExpression(CallExpression::new(callee, args))
}

fn syntax_check(buf: &[TokenType]) -> ProcedureParserState {
    let first = buf
        .get(0)
        .expect("Procedure Parser accept only one token")
        .expect_keywords();
    let last = buf
        .last()
        .expect("Procedure Parser accept only one token")
        .expect_keywords();
    match (&first, &last) {
        (Some(KeywordType::To), Some(KeywordType::End)) => ProcedureParserState::Define,
        (Some(KeywordType::To), None) => panic!(
            "Procedure Parser accept only TO/END KeywordType token, but got {:?} and {:?}",
            first, last
        ),
        (None, Some(KeywordType::End)) => panic!(
            "Procedure Parser accept only TO/END KeywordType token, but got {:?} and {:?}",
            first, last
        ),
        _ => ProcedureParserState::Call,
    }
}
