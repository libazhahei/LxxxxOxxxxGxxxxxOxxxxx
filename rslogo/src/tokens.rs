use std::{
    collections::VecDeque,
    mem::swap,
    ops::{Add, Div, Mul, Sub},
};

use crate::{ast::Statement, parsers::ParserFactory};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum ValueType {
    Float(f32),
    Int(i32),
    Bool(bool),
}

impl Mul for ValueType {
    type Output = ValueType;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueType::Int(lhs), ValueType::Int(rhs)) => ValueType::Int(lhs * rhs),
            (ValueType::Float(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs * rhs),
            (ValueType::Int(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs as f32 * rhs),
            (ValueType::Float(lhs), ValueType::Int(rhs)) => ValueType::Float(lhs * rhs as f32),
            _ => panic!("Unsupported multiplication operation"),
        }
    }
}
impl Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => ValueType::Float(f1 + f2),
            (ValueType::Int(i1), ValueType::Int(i2)) => ValueType::Int(i1 + i2),
            (ValueType::Float(f1), ValueType::Int(i2)) => ValueType::Float(f1 + (i2 as f32)),
            (ValueType::Int(i1), ValueType::Float(f2)) => ValueType::Float(i1 as f32 + f2),
            _ => panic!("Unsupported addition operation"),
        }
    }
}
impl Div for ValueType {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueType::Int(lhs), ValueType::Int(rhs)) => ValueType::Int(lhs / rhs),
            (ValueType::Float(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs / rhs),
            (ValueType::Int(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs as f32 / rhs),
            (ValueType::Float(lhs), ValueType::Int(rhs)) => ValueType::Float(lhs / rhs as f32),
            _ => panic!("Unsupported division operation"),
        }
    }
}

impl Sub for ValueType {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueType::Int(lhs), ValueType::Int(rhs)) => ValueType::Int(lhs - rhs),
            (ValueType::Float(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs - rhs),
            (ValueType::Int(lhs), ValueType::Float(rhs)) => ValueType::Float(lhs as f32 - rhs),
            (ValueType::Float(lhs), ValueType::Int(rhs)) => ValueType::Float(lhs - rhs as f32),
            _ => panic!("Unsupported subtraction operation"),
        }
    }
}
impl PartialEq for ValueType {
    // type Output = Self;
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1 == f2,
            (ValueType::Int(i1), ValueType::Int(i2)) => i1 == i2,
            (ValueType::Float(f1), ValueType::Int(i2)) => *f1 == *i2 as f32,
            (ValueType::Int(i1), ValueType::Float(f2)) => *i1 as f32 == *f2,
            (ValueType::Bool(b1), ValueType::Bool(b2)) => *b1 == *b2,
            _ => panic!("Unsupported comparison operation"),
        }
    }

    // fn ne(&self, other: &Self) -> bool {
    //     match (self, other) {
    //         (ValueType::Float(f1), ValueType::Float(f2)) => f1 != f2,
    //         (ValueType::Int(i1), ValueType::Int(i2)) => i1 != i2,
    //         (ValueType::Float(f1), ValueType::Int(i2)) => *f1 != *i2 as f32,
    //         (ValueType::Int(i1), ValueType::Float(f2)) => *i1 as f32 != *f2,
    //         (ValueType::Bool(b1), ValueType::Bool(b2)) => *b1 != *b2,
    //         _ => panic!("Unsupported comparison operation"),
    //     }
    // }
}

impl PartialOrd for ValueType {
    // type Output = Self;

    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1 >= f2,
            (ValueType::Int(i1), ValueType::Int(i2)) => i1 >= i2,
            (ValueType::Float(f1), ValueType::Int(i2)) => *f1 >= *i2 as f32,
            (ValueType::Int(i1), ValueType::Float(f2)) => (*i1 as f32) >= *f2,
            _ => panic!("Unsupported comparison operation"),
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1 > f2,
            (ValueType::Int(i1), ValueType::Int(i2)) => i1 > i2,
            (ValueType::Float(f1), ValueType::Int(i2)) => *f1 > *i2 as f32,
            (ValueType::Int(i1), ValueType::Float(f2)) => (*i1 as f32) > *f2,
            _ => panic!("Unsupported comparison operation"),
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1 <= f2,
            (ValueType::Int(i1), ValueType::Int(i2)) => i1 <= i2,
            (ValueType::Float(f1), ValueType::Int(i2)) => *f1 <= *i2 as f32,
            (ValueType::Int(i1), ValueType::Float(f2)) => (*i1 as f32) <= *f2,
            _ => panic!("Unsupported comparison operation"),
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1 < f2,
            (ValueType::Int(i1), ValueType::Int(i2)) => i1 < i2,
            (ValueType::Float(f1), ValueType::Int(i2)) => *f1 < *i2 as f32,
            (ValueType::Int(i1), ValueType::Float(f2)) => (*i1 as f32) < *f2,
            _ => panic!("Unsupported comparison operation"),
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ValueType::Float(f1), ValueType::Float(f2)) => f1.partial_cmp(f2),
            (ValueType::Int(i1), ValueType::Int(i2)) => i1.partial_cmp(i2),
            (ValueType::Float(f1), ValueType::Int(i2)) => f1.partial_cmp(&(*i2 as f32)),
            (ValueType::Int(i1), ValueType::Float(f2)) => (*i1 as f32).partial_cmp(f2),
            _ => panic!("Unsupported comparison operation"),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum IdentifierType {
    Variable(String),
    Procedure(String),
}
impl IdentifierType {
    pub fn expect_variable(&self) -> Option<String> {
        match self {
            IdentifierType::Variable(name) => Some(name.clone()),
            _ => None,
        }
    }
    pub fn expect_procedure(&self) -> Option<String> {
        match self {
            IdentifierType::Procedure(name) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn is_procedure(&self) -> bool {
        matches!(self, IdentifierType::Procedure(_))
    }
}

#[derive(Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub enum AssignmentType {
    Make,
    Addassign,
}

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Debug)]
pub enum CommandType {
    Penup,
    Pendown,
    Forward,
    Back,
    Left,
    Right,
    Setpencolor,
    Turn,
    Setheading,
    Setx,
    Sety,
}

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Debug)]
pub enum QueryType {
    Xcor,
    Ycor,
    Heading,
    Color,
}
#[derive(Clone, Debug, Serialize)]
pub enum KeywordType {
    If,
    While,
    To,
    End,
}

#[derive(Clone, Debug, Serialize)]
pub enum LogicalOperation {
    Eq,  // == / Eq
    Ne,  // != / Ne
    Gt,  // > / GE
    Lt,  // < / Lt
    And, // == / And
    Or,  // || / Or
}

#[derive(Clone, Debug, Serialize)]
pub enum CalculationOperation {
    Plus,  // +
    Dash,  // -
    Star,  // *
    Slash, // /
}

#[derive(Clone, Debug, Serialize)]
pub enum TokenType {
    Bracketleft,  // [
    Bracketright, // ]
    Space,
    // Variable(String),
    Value(ValueType),
    Identifier(IdentifierType),
    Command(CommandType),
    Query(QueryType),
    Keyword(KeywordType),
    Logic(LogicalOperation),
    Calculation(CalculationOperation),
    Assignment(AssignmentType),
}
impl TokenType {
    pub fn expect_identifier(&self) -> Option<IdentifierType> {
        match self {
            TokenType::Identifier(identifier) => Some(identifier.clone()),
            _ => None,
        }
    }

    pub fn expect_command(&self) -> Option<CommandType> {
        match self {
            TokenType::Command(command) => Some(command.clone()),
            _ => None,
        }
    }
    pub fn expect_assignment(&self) -> Option<AssignmentType> {
        match self {
            TokenType::Assignment(assignment) => Some(assignment.clone()),
            _ => None,
        }
    }
    pub fn expect_keywords(&self) -> Option<KeywordType> {
        match self {
            TokenType::Keyword(keywords) => Some(keywords.clone()),
            _ => None,
        }
    }
    pub fn is_left_bracket(&self) -> bool {
        matches!(self, TokenType::Bracketleft)
    }

    pub fn is_right_bracket(&self) -> bool {
        matches!(self, TokenType::Bracketright)
    }
    pub fn is_space(&self) -> bool {
        matches!(self, TokenType::Space)
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, TokenType::Calculation(_) | TokenType::Logic(_))
    }
}

pub struct Tokenizer<'a> {
    tokens: VecDeque<TokenType>,
    logo: &'a String,
}

impl Tokenizer<'_> {
    pub fn new(logo: &String) -> Tokenizer {
        Tokenizer {
            tokens: VecDeque::new(),
            logo,
        }
    }

    pub fn scan(&mut self) {
        for line in self.logo.lines() {
            if line.trim().starts_with("//") {
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            line.trim().split_ascii_whitespace().for_each(|token| {
                self.scan_token(token);
            });
            self.tokens.push_back(TokenType::Space)
        }
    }
    pub fn move_token(&mut self) -> VecDeque<TokenType> {
        let mut tokens: VecDeque<TokenType> = VecDeque::new();
        swap(&mut self.tokens, &mut tokens);
        tokens
    }

    fn scan_token(&mut self, token: &str) {
        if let Some(keywords) = Tokenizer::scan_keywords(token) {
            self.tokens.push_back(keywords)
        } else if let Some(variable) = Tokenizer::scan_variable(token) {
            self.tokens.push_back(variable)
        } else if let Some(value) = Tokenizer::scan_value(token) {
            self.tokens.push_back(value)
        } else if let Some(char) = Tokenizer::scan_char(token) {
            self.tokens.push_back(char)
        } else {
            self.tokens
                .push_back(TokenType::Identifier(IdentifierType::Procedure(
                    token.to_string(),
                )))
        }
    }

    fn scan_keywords(token: &str) -> Option<TokenType> {
        match token {
            "PENUP" => Some(TokenType::Command(CommandType::Penup)),
            "PENDOWN" => Some(TokenType::Command(CommandType::Pendown)),
            "FORWARD" => Some(TokenType::Command(CommandType::Forward)),
            "BACK" => Some(TokenType::Command(CommandType::Back)),
            "LEFT" => Some(TokenType::Command(CommandType::Left)),
            "RIGHT" => Some(TokenType::Command(CommandType::Right)),
            "SETPENCOLOR" => Some(TokenType::Command(CommandType::Setpencolor)),
            "TURN" => Some(TokenType::Command(CommandType::Turn)),
            "SETHEADING" => Some(TokenType::Command(CommandType::Setheading)),
            "SETX" => Some(TokenType::Command(CommandType::Setx)),
            "SETY" => Some(TokenType::Command(CommandType::Sety)),
            "MAKE" => Some(TokenType::Assignment(AssignmentType::Make)),
            "ADDASSIGN" => Some(TokenType::Assignment(AssignmentType::Addassign)),
            "XCOR" => Some(TokenType::Query(QueryType::Xcor)),
            "YCOR" => Some(TokenType::Query(QueryType::Ycor)),
            "HEADING" => Some(TokenType::Query(QueryType::Heading)),
            "COLOR" => Some(TokenType::Query(QueryType::Color)),
            "IF" => Some(TokenType::Keyword(KeywordType::If)),
            "WHILE" => Some(TokenType::Keyword(KeywordType::While)),
            "TO" => Some(TokenType::Keyword(KeywordType::To)),
            "END" => Some(TokenType::Keyword(KeywordType::End)),
            "EQ" => Some(TokenType::Logic(LogicalOperation::Eq)),
            "NE" => Some(TokenType::Logic(LogicalOperation::Ne)),
            "GT" => Some(TokenType::Logic(LogicalOperation::Gt)),
            "LT" => Some(TokenType::Logic(LogicalOperation::Lt)),
            "AND" => Some(TokenType::Logic(LogicalOperation::And)),
            "OR" => Some(TokenType::Logic(LogicalOperation::Or)),
            _ => None,
        }
    }

    fn scan_variable(token: &str) -> Option<TokenType> {
        if token.starts_with(':') {
            Some(TokenType::Identifier(IdentifierType::Variable(
                token.strip_prefix(':').unwrap().to_string(),
            )))
        } else {
            None
        }
    }

    fn scan_value(token: &str) -> Option<TokenType> {
        if token.starts_with('"') {
            let value = token.strip_prefix('"').unwrap();
            if value.parse::<i32>().is_ok() {
                Some(TokenType::Value(ValueType::Int(
                    value.parse::<i32>().unwrap(),
                )))
            } else if value.parse::<f32>().is_ok() {
                Some(TokenType::Value(ValueType::Float(
                    value.parse::<f32>().unwrap(),
                )))
            } else if value.to_ascii_lowercase().parse::<bool>().is_ok() {
                Some(TokenType::Value(ValueType::Bool(
                    value.to_ascii_lowercase().parse::<bool>().unwrap(),
                )))
            } else {
                Some(TokenType::Identifier(IdentifierType::Variable(
                    value.to_string(),
                )))
            }
        } else {
            None
        }
    }

    fn scan_char(token: &str) -> Option<TokenType> {
        match token {
            "[" => Some(TokenType::Bracketleft),
            "]" => Some(TokenType::Bracketright),
            "+" => Some(TokenType::Calculation(CalculationOperation::Plus)),
            "-" => Some(TokenType::Calculation(CalculationOperation::Dash)),
            "*" => Some(TokenType::Calculation(CalculationOperation::Star)),
            "/" => Some(TokenType::Calculation(CalculationOperation::Slash)),
            "\n" => Some(TokenType::Space),
            "\t" => Some(TokenType::Space),
            "\r\n" => Some(TokenType::Space),
            _ => None,
        }
    }
}

pub trait StatementParser {
    fn parse(&self, buf: &[TokenType]) -> Statement;
}
pub struct ASTParser {
    tokens: VecDeque<TokenType>,
}

impl ASTParser {
    pub fn new(logo: &String) -> ASTParser {
        let mut tokenizer = Tokenizer::new(logo);
        tokenizer.scan();
        ASTParser {
            tokens: tokenizer.move_token(),
        }
    }
    pub fn new_from_tokens(tokens: VecDeque<TokenType>) -> ASTParser {
        ASTParser { tokens }
    }

    pub fn parse_from_tokens(tokens: &[TokenType]) -> Vec<Statement> {
        let mut parser = ASTParser::new_from_tokens(tokens.to_vec().into());
        parser.parse_statement()
    }

    pub fn parse_statement(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = self.tokens.pop_front() {
            let mut buf: Vec<TokenType> = Vec::new();
            match &token {
                TokenType::Keyword(keyword) => {
                    buf.push(token.clone());
                    match keyword {
                        KeywordType::If | KeywordType::While => {
                            while let Some(token) = self.tokens.pop_front() {
                                if token.is_left_bracket() {
                                    buf.push(token);
                                    break;
                                }
                                buf.push(token);
                            }
                            let mut num_left_bracket = 1;
                            while let Some(token) = self.tokens.pop_front() {
                                if token.is_left_bracket() {
                                    num_left_bracket += 1;
                                }
                                if token.is_right_bracket() {
                                    num_left_bracket -= 1;
                                }
                                if num_left_bracket == 0 {
                                    buf.push(token);
                                    self.tokens.pop_front();
                                    break;
                                }
                                buf.push(token);
                            }
                        }
                        KeywordType::To => {
                            while let Some(token) = self.tokens.pop_front() {
                                if let TokenType::Keyword(KeywordType::End) = &token {
                                    buf.push(token);
                                    self.tokens.pop_front();
                                    break;
                                };
                                buf.push(token);
                            }
                        }
                        _ => panic!("Missing To Keyword before End"),
                    }
                }
                _ => {
                    buf.push(token);
                    while let Some(token) = self.tokens.pop_front() {
                        match token {
                            TokenType::Space => {
                                if !buf.is_empty() {
                                    break;
                                }
                            }
                            _ => buf.push(token),
                        }
                    }
                }
            };
            statements.push(ParserFactory::create(&buf).parse(&buf));
        }
        statements
    }
}
