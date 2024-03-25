use crate::{
    commands::Value,
    tokens::{KeywordType, TokenType},
};

use self::{binary::BinaryParser, queries::QueriesParser};

mod binary;
mod controls;
mod ifwhile;
mod procedures;
mod queries;
mod variable;
use crate::tokens::StatementParser;

impl TokenType {
    pub fn to_value(&self) -> Option<Value> {
        match self {
            TokenType::Identifier(identifier) => Some(Value::Identifier(identifier.clone())),
            TokenType::Value(value) => Some(Value::Literal(value.clone())),
            TokenType::Query(query) => {
                let query_token: Vec<TokenType> = vec![TokenType::Query(query.clone())];
                Some(Value::Statement(Box::new(
                    QueriesParser {}
                        .parse(&query_token)
                        .expect_callexpression()?,
                )))
            }
            // TokenType::CALCULATION(calculation) => ,
            // TokenType::LOGIC(logic) => todo!(""),
            _ => None,
        }
    }
}

pub struct ParserFactory {}
impl ParserFactory {
    pub fn create(tokens: &Vec<TokenType>) -> Box<dyn StatementParser> {
        // println!("ParserFactory: Parsing Token '{:?}'", tokens);
        let first = tokens
            .get(0)
            .expect("ParserFactory::create accept only non-empty tokens");
        match first {
            TokenType::Assignment(_) => Box::new(variable::VariableAssignParser {}),
            TokenType::Command(_) => Box::new(controls::ControlsParser {}),
            TokenType::Keyword(keywords) => match keywords {
                KeywordType::If => Box::new(ifwhile::IfStatementParser {}),
                KeywordType::While => Box::new(ifwhile::WhileStatementParser {}),
                KeywordType::To => Box::new(procedures::ProcedureParser {}),
                _ => panic!("ParserFactory::create accept only non-empty tokens"),
            },
            TokenType::Identifier(identifier) => {
                if identifier.is_procedure() {
                    Box::new(procedures::ProcedureParser {})
                } else {
                    panic!(
                        "ParserFactory Error: {:?}/{:?} : {:?}",
                        first, identifier, tokens
                    )
                }
            }
            _ => panic!("ParserFactory Error: {:?} : {:?}", first, tokens),
        }
    }
}

impl Value {
    fn parse_one(buf_itr: &mut std::slice::Iter<'_, TokenType>) -> Value {
        let token = buf_itr.next().expect("buffer iterator is empty");
        let value = token.to_value();
        if value.is_none() {
            return Value::BinaryExpression(Box::new(
                BinaryParser::_parse(token, buf_itr).expect("It is not an accept data type"),
            ));
        }
        value.unwrap()
    }
}
