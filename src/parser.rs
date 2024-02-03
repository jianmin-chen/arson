use crate::ast::{
    new_array, new_attr, new_binop, new_bool, new_call, new_chain, new_class, new_dict, new_else,
    new_for, new_func, new_if, new_number, new_return, new_string, new_var, new_while, AstValues,
    Node, Number,
};
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

fn stmt(parser: &mut Parser) -> Node {
    let curr = parser.peek_token_type().unwrap().clone();
    match curr {
        TokenType::Var => return var_stmt(parser),
        TokenType::Call => return call(parser),
        TokenType::Func => return func_stmt(parser),
        TokenType::Class => return class_stmt(parser),
        TokenType::Return => return return_stmt(parser),
        TokenType::For => return for_stmt(parser),
        TokenType::While => return while_stmt(parser),
        TokenType::If => return if_stmt(parser),
        _ => return expr(parser, None),
    }
}

fn simple(parser: &mut Parser) -> Node {
    let token = parser
        .eat(parser.peek_token_type().unwrap().clone())
        .unwrap();

    fn negative(ast_node: Node) -> AstValues {
        let number = cast!(ast_node.get("value").unwrap(), AstValues::Number);
        match number {
            Number::Int(value) => return AstValues::Number(Number::Int(value * -1)),
            Number::Float(value) => return AstValues::Number(Number::Float(value * -1.0)),
        }
    }

    fn to_number(val: String) -> AstValues {
        // Token value is a String: convert it to AstValues::Number(Number::type(value))
        if val.contains('.') {
            return AstValues::Number(Number::Float(val.parse::<f64>().unwrap()));
        }
        return AstValues::Number(Number::Int(val.parse::<isize>().unwrap()));
    }

    match token.kind {
        TokenType::Word => return new_var(token.value.clone(), None),
        TokenType::Minus => return new_number(negative(simple(parser))),
        TokenType::Number => return new_number(to_number(token.value.clone())),
        TokenType::String => return new_string(AstValues::String(token.value.clone())),
        TokenType::True => return new_bool(AstValues::Bool(true)),
        TokenType::False => return new_bool(AstValues::Bool(false)),
        // TokenType::New => {
        //   // New instance
        //   let id = parser.eat(TokenType::Word);
        // },
        TokenType::LeftParen => {
            // Left parentheses
            let left: Node = expr(parser, Some(true));
            parser.eat(TokenType::RightParen);
            return left;
        }
        TokenType::LeftBracket => {
            let mut items: Vec<Node> = Vec::new();
            if *parser.peek_token_type().unwrap() != TokenType::RightBracket {
                items.push(expr(parser, None));
                while *parser.peek_token_type().unwrap() == TokenType::Comma {
                    parser.eat(TokenType::Comma);
                    items.push(expr(parser, None));
                }
            }
            parser.eat(TokenType::RightBracket);
            return new_array(items);
        }
        TokenType::LeftBrace => {
            let mut obj: HashMap<String, AstValues> = HashMap::new();
            while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
                let key: String = parser.eat(TokenType::String).unwrap().clone().value;
                parser.eat(TokenType::Colon);
                obj.insert(key, AstValues::Node(expr(parser, None)));
                if *parser.peek_token_type().unwrap() != TokenType::RightBrace {
                    parser.eat(TokenType::Comma);
                }
            }
            parser.eat(TokenType::RightBrace);
            return new_dict(obj);
        }
        _ => panic!("Expected expression but got {:?}", token.kind),
    }
}

fn is_op(kind: TokenType) -> bool {
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
    return ops.contains(&kind);
}

fn call(parser: &mut Parser) -> Node {
    let res: Node = simple(parser);
    if *parser.peek_token_type().unwrap() == TokenType::LeftParen
        || *parser.peek_token_type().unwrap() == TokenType::LeftBracket
    {
        let mut chain: Vec<Node> = Vec::new();
        while *parser.peek_token_type().unwrap() == TokenType::LeftParen
            || *parser.peek_token_type().unwrap() == TokenType::LeftBracket
        {
            if *parser.peek_token_type().unwrap() == TokenType::LeftParen {
                parser.eat(TokenType::LeftParen);
                let args: Vec<Node> = expr_list(parser);
                parser.eat(TokenType::RightParen);
                chain.push(new_call(args));
            } else {
                parser.eat(TokenType::LeftBracket);
                if *parser.peek_token_type().unwrap() == TokenType::Period {
                    parser.eat(TokenType::Period);
                    let id = parser
                        .eat(parser.peek_token_type().unwrap().clone())
                        .unwrap()
                        .clone();
                    chain.push(new_attr(id.value.clone()));
                } else {
                    chain.push(expr(parser, None));
                }
                parser.eat(TokenType::RightBracket);
            }
        }
        return new_chain(res, chain);
    }
    res
}

fn expr(parser: &mut Parser, wrapped: Option<bool>) -> Node {
    let wrapped = wrapped.unwrap_or(false);
    let left: Node = call(parser);
    if is_op(parser.peek_token_type().unwrap().clone()) {
        let op: String = parser
            .eat(parser.peek_token_type().unwrap().clone())
            .unwrap()
            .clone()
            .value;
        let right: Node = expr(parser, None);
        return new_binop(left, right, op, Some(wrapped));
    }
    left
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

fn expr_list(parser: &mut Parser) -> Vec<Node> {
    // Expressions in a group ()
    let mut exprs: Vec<Node> = Vec::new();
    if *parser.peek_token_type().unwrap() != TokenType::RightParen {
        exprs.push(expr(parser, None));
        while *parser.peek_token_type().unwrap() == TokenType::Comma {
            parser.eat(TokenType::Comma);
            exprs.push(expr(parser, None));
        }
    }
    exprs
}

fn var_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Var);
    let id: String = parser.eat(TokenType::Word).unwrap().clone().value;
    parser.eat(TokenType::Equal);
    let value = expr(parser, None);
    new_var(id, Some(AstValues::Node(value)))
}

fn func_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Func);
    let id: String = parser.eat(TokenType::Word).unwrap().value.clone();
    parser.eat(TokenType::LeftParen);
    let params: Vec<String> = id_list(parser);
    parser.eat(TokenType::RightParen);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_func(id, params, body)
}

fn class_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Class);
    let id = parser.eat(TokenType::Word).unwrap().value.clone();
    parser.eat(TokenType::LeftBrace);
    let mut methods: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() == TokenType::Func {
        methods.push(func_stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_class(id, methods)
}

fn return_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Return);
    let value: Node = expr(parser, None);
    new_return(value)
}

fn if_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::If);
    parser.eat(TokenType::LeftParen);
    let condition = stmt(parser);
    parser.eat(TokenType::RightParen);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    let mut otherwise: Vec<Node> = Vec::new();
    if *parser.peek_token_type().unwrap() == TokenType::Else {
        otherwise.push(else_stmt(parser));
    }
    while *parser.peek_token_type().unwrap() == TokenType::Elif {
        otherwise.push(elif_stmt(parser));
    }
    if *parser.peek_token_type().unwrap() == TokenType::Else {
        otherwise.push(else_stmt(parser));
    }
    new_if(condition, body, Some(otherwise))
}

fn elif_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Elif);
    parser.eat(TokenType::LeftParen);
    let condition: Node = stmt(parser);
    parser.eat(TokenType::RightParen);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_if(condition, body, None)
}

fn else_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::Else);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_else(body)
}

fn for_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::For);
    let id: String = parser.eat(TokenType::Word).unwrap().clone().value;
    parser.eat(TokenType::Range);
    parser.eat(TokenType::LeftParen);
    let through: Vec<Node> = expr_list(parser);
    parser.eat(TokenType::RightParen);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_for(id, through, body)
}

fn while_stmt(parser: &mut Parser) -> Node {
    parser.eat(TokenType::While);
    parser.eat(TokenType::LeftParen);
    let condition: Node = expr(parser, None);
    parser.eat(TokenType::RightParen);
    parser.eat(TokenType::LeftBrace);
    let mut body: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::RightBrace {
        body.push(stmt(parser));
    }
    parser.eat(TokenType::RightBrace);
    new_while(condition, body)
}

pub fn program(parser: &mut Parser) -> Vec<Node> {
    let mut parsed: Vec<Node> = Vec::new();
    while *parser.peek_token_type().unwrap() != TokenType::Eof {
        parsed.push(stmt(parser));
    }
    parsed
}
