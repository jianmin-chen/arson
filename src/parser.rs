use crate::ast::{new_bool, new_number, new_string, new_var, AstType, AstValues};
use crate::lexer::{Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if self.current >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.current])
    }

    pub fn peek_token_type(&self) -> Option<&TokenType> {
        if self.current >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.current].kind)
    }

    pub fn eat(&mut self, kind: TokenType) -> Option<&Token> {
        if let Some(token_kind) = self.peek_token_type() {
            if *token_kind == kind {
                let res = &self.tokens[self.current];
                self.current += 1;
                return Some(res);
            } else {
                panic!("Expected {:?} but got {:?}", kind, token_kind);
            }
        } else {
            panic!("Expected token but got None");
        }
    }
}

fn stmt(parser: &mut Parser) {
    let curr = parser.peek_token_type().unwrap().clone();
    match curr {
        TokenType::Var => {}
        TokenType::Call => {}
        TokenType::Func => {}
        TokenType::Class => {}
        TokenType::Return => {}
        TokenType::For => {}
        TokenType::While => {}
        TokenType::If => {}
        _ => {}
    }
}

fn simple(parser: &mut Parser) -> HashMap<&'static str, AstValues> {
    let token = parser
        .eat(parser.peek_token_type().unwrap().clone())
        .unwrap();

    fn negative(val: AstValues) -> AstValues {
        AstValues::Number((val as isize) * -1)
    }

    match token.kind {
        TokenType::Word => return new_var(token.value, None),
        TokenType::Minus => return new_number(negative(*simple(parser).get("value").unwrap())),
        TokenType::Number => {
            return new_number(AstValues::Number(token.value.parse::<isize>().unwrap()))
        }
        TokenType::String => return new_string(AstValues::String(token.value)),
        TokenType::True => return new_bool(AstValues::Bool(true)),
        TokenType::False => return new_bool(AstValues::Bool(false)),
        // TokenType::New => {
        //   // New instance
        //   let id = parser.eat(TokenType::Word);

        // },
        // TokenType::LeftParen => {
        //     // Left parentheses
        //     let left = expr(parser, True);
        // }
        // TokenType::LeftBracket => {}
        // TokenType::LeftBrace => {}
        _ => panic!("Expected expression but got {:?}", token.kind),
    }
}

fn is_op(token: Token) -> bool {
    let ops = [
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Times,
        TokenType::Divide,
        TokenType::Modulo,
        TokenType::LessThan,
        TokenType::LessThanOrEqual,
        TokenType::GreaterThan,
        TokenType::GreaterThanOrEqual,
        TokenType::Equality,
        TokenType::Equal,
        TokenType::And,
        TokenType::Or,
    ];
    return ops.contains(&token.kind);
}

fn call(parser: &mut Parser) {}

fn expr(parser: &mut Parser, wrapped: Option<bool>) {
    let wrapped = wrapped.unwrap_or(false);
    // let left = call(parser);
}

fn id_list(parser: &mut Parser) -> Vec<String> {
    // References in a group ()
    let mut values = Vec::new();
    if *parser.peek_token_type().unwrap() == TokenType::Word {
        values.push(parser.eat(TokenType::Word).unwrap().value.clone());
        while *parser.peek_token_type().unwrap() == TokenType::Comma {
            parser.eat(TokenType::Comma);
            values.push(parser.eat(TokenType::Word).unwrap().value.clone());
        }
    }
    values
}

fn expr_list(parser: &mut Parser) -> Vec<String> {
    // Expressions in a group
    let mut exprs: Vec<String> = Vec::new();
    if *parser.peek_token_type().unwrap() != TokenType::RightParen {}
    exprs
}

fn var_stmt() {}

fn class_stmt() {}

fn return_stmt() {}

fn if_stmt() {}

fn elif_stmt() {}

fn else_stmt() {}

fn for_stmt() {}

fn while_stmt() {}

fn program(parser: &mut Parser) {}
