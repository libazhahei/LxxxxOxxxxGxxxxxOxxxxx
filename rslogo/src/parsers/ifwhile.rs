use crate::{
    ast::Statement,
    ast::{IfStatement, TestType, WhileStatement},
    parsers::binary::BinaryParser,
    tokens::{ASTParser, StatementParser, TokenType},
};

pub struct IfStatementParser {}
impl StatementParser for IfStatementParser {
    fn parse(&self, buf: &[TokenType]) -> Statement {
        // IfStatementParser::parse_if(buf)
        let (test, body) = parse_if_while(buf);
        Statement::If(IfStatement::new(test, body))
    }
}
pub struct WhileStatementParser {}
impl StatementParser for WhileStatementParser {
    fn parse(&self, buf: &[TokenType]) -> Statement {
        let (test, body) = parse_if_while(buf);
        Statement::While(WhileStatement::new(test, body))
    }
}

fn parse_if_while(buf: &[TokenType]) -> (TestType, Vec<Statement>) {
    syntax_check(buf);
    let itr = buf.iter().skip(1);
    let condi: Vec<TokenType> = itr
        .take_while(|token| !token.is_left_bracket())
        .cloned()
        .collect();
    // println!("CONDI : {:?}", condi);
    let test = BinaryParser::parse(&condi);
    let itr = buf
        .iter()
        .skip_while(|token| !token.is_left_bracket())
        .skip(2);
    let mut body: Vec<TokenType> = itr.cloned().collect();
    body.pop();
    // println!("BODY : {:?}", body);
    let body = ASTParser::parse_from_tokens(&body);
    (test, body)
}
fn syntax_check(buf: &[TokenType]) {
    let num_left_bracket = buf.iter().filter(|token| token.is_left_bracket()).count();
    let num_right_bracket = buf.iter().filter(|token| token.is_right_bracket()).count();
    if num_left_bracket == 0 {
        panic!("Syntax Error: Missing left bracket");
    }
    if num_left_bracket != num_right_bracket {
        panic!(
            "Syntax Error: Bracket number is not match, left : {}, right : {}",
            num_left_bracket, num_right_bracket
        );
    }
}
