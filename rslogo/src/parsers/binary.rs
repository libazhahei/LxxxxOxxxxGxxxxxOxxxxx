use crate::{
    ast::{BinaryExpression, TestType},
    commands::Value,
    tokens::TokenType,
};

pub struct BinaryParser {}

impl BinaryParser {
    pub fn parse(buf: &[TokenType]) -> TestType {
        let mut itr = buf.iter();
        Value::parse_one(&mut itr)
    }
    pub fn _parse(
        first: &TokenType,
        token: &mut std::slice::Iter<'_, TokenType>,
    ) -> Option<BinaryExpression> {
        if !first.is_binary() {
            return None;
        }
        let left = token
            .next()
            .expect("error in number of variable for binary expression");
        let left_parse = BinaryParser::_parse(left, token);
        let right = token
            .next()
            .expect("error in number of variable for binary expression");
        let right_parse = BinaryParser::_parse(right, token);

        match (left_parse.is_some(), right_parse.is_some()) {
            (true, true) => Some(BinaryExpression::new(
                first.clone(),
                Value::BinaryExpression(Box::new(left_parse.unwrap())),
                Value::BinaryExpression(Box::new(right_parse.unwrap())),
            )),
            (true, false) => Some(BinaryExpression::new(
                first.clone(),
                Value::BinaryExpression(Box::new(left_parse.unwrap())),
                right.to_value().expect("unsupport to convert to Value"),
            )),
            (false, true) => Some(BinaryExpression::new(
                first.clone(),
                left.to_value().expect("unsupport to convert to Value"),
                Value::BinaryExpression(Box::new(right_parse.unwrap())),
            )),
            (false, false) => Some(BinaryExpression::new(
                first.clone(),
                left.to_value().expect("unsupport to convert to Value"),
                right.to_value().expect("unsupport to convert to Value"),
            )),
        }
    }
}
