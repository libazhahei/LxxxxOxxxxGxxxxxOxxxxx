use crate::{
    ast::Statement,
    ast::{CallExpression, Callee},
    tokens::{StatementParser, TokenType},
};

pub struct QueriesParser {}
impl StatementParser for QueriesParser {
    fn parse(&self, tokens: &[TokenType]) -> Statement {
        QueriesParser::parse_queries(tokens)
    }
}

impl QueriesParser {
    fn parse_queries(tokens: &[TokenType]) -> Statement {
        if tokens.len() != 1 {
            panic!("Queries Parser accept only one token");
        }
        let query = tokens.get(0).expect("Queries Parser accept only one token");
        let expression: CallExpression = match query {
            TokenType::Query(query) => {
                CallExpression::new(Callee::Query(query.clone()), Vec::new())
            }
            _ => panic!("Queries Parser accept only one QueryType token"),
        };
        Statement::CallExpression(expression)
    }
}
