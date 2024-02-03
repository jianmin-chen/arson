use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Word,
    Number,
    String,
    Array,
    Dict,
    Func,
    Call,
    Var,
    If,
    Elif,
    Else,
    While,
    For,
    True,
    False,
    And,
    Or,
    Return,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Equal,
    Equality,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Eof,
    Class,
    New,
    Range,
    Colon,
    Period,
}

pub fn keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("return", TokenType::Return);
    keywords.insert("burn", TokenType::Var);
    keywords.insert("for", TokenType::For);
    keywords.insert("through", TokenType::Range);
    keywords.insert("prepmatch", TokenType::Func);
    keywords.insert("while", TokenType::While);
    keywords.insert("lightertype", TokenType::Class);
    keywords.insert("pullout", TokenType::New);
    keywords.insert("if", TokenType::If);
    keywords.insert("elif", TokenType::Elif);
    keywords.insert("else", TokenType::Else);
    keywords.insert("True", TokenType::True);
    keywords.insert("False", TokenType::False);
    keywords.insert("and", TokenType::And);
    keywords.insert("or", TokenType::Or);
    return keywords;
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub content: String,
}

impl Token {
    pub fn new(kind: TokenType, value: String, content: String) -> Token {
        Token {
            kind,
            value,
            content,
        }
    }
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    current: usize,
    line: usize,
}

fn char_at(s: &String, index: usize) -> Option<char> {
    s.chars().nth(index)
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 0,
        }
    }

    pub fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        char_at(&self.source, self.current).unwrap()
    }

    pub fn peek_next(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        char_at(&self.source, self.current + 1).unwrap()
    }

    pub fn advance(&mut self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        let res = self.peek();
        self.current += 1;
        res
    }

    pub fn r#match(&mut self, chr: char) -> bool {
        if self.peek() == chr {
            self.advance();
            return true;
        }
        false
    }

    pub fn add_token(&mut self, kind: TokenType, value: String, content: String) {
        self.tokens.push(Token::new(kind, value, content))
    }
}

pub fn scan_token(lexer: &mut Lexer) {
    let chr = lexer.advance();

    fn string(kind: char, lexer: &mut Lexer) {
        let mut text = String::new();
        while lexer.peek() != kind && lexer.peek() != '\0' {
            if lexer.peek() == '\n' {
                lexer.line += 1;
            }
            text.push_str(&lexer.advance().to_string());
        }
        if lexer.peek() == '\0' {
            // Reached end of file, but string hasn't been terminated
            panic!("Unterminated string: {}", lexer.line);
        }
        lexer.advance(); // Consume the closing quote
        lexer.add_token(TokenType::String, text.clone(), text.clone());
    }

    fn number(lexer: &mut Lexer) {
        let mut text = String::new();
        while lexer.peek().is_numeric() {
            text.push(lexer.advance());
        }
        if lexer.peek() == '.' && lexer.peek_next().is_numeric() {
            text.push(lexer.advance());
            while lexer.peek().is_numeric() {
                text.push(lexer.advance());
            }
        }
        lexer.add_token(TokenType::Number, text.clone(), text.clone());
    }

    fn identifier(lexer: &mut Lexer) {
        let mut text = String::new();
        while lexer.peek().is_alphabetic() {
            text.push(lexer.advance());
        }
        match keywords().get(&text.as_str()) {
            Some(value) => {
                lexer.add_token(value.clone(), text.clone(), text.clone());
            }
            None => {
                lexer.add_token(TokenType::Word, text.clone(), text.clone());
            }
        }
    }

    match chr {
        '(' => lexer.add_token(TokenType::LeftParen, String::from("("), String::from("(")),
        ')' => lexer.add_token(TokenType::RightParen, String::from(")"), String::from(")")),
        '{' => lexer.add_token(TokenType::LeftBrace, String::from("{"), String::from("{")),
        '}' => lexer.add_token(TokenType::RightBrace, String::from("}"), String::from("}")),
        '[' => lexer.add_token(TokenType::LeftBracket, String::from("["), String::from("[")),
        ']' => lexer.add_token(
            TokenType::RightBracket,
            String::from("]"),
            String::from("]"),
        ),
        ',' => lexer.add_token(TokenType::Comma, String::from(","), String::from(",")),
        '+' => lexer.add_token(TokenType::Plus, String::from("+"), String::from("+")),
        '-' => lexer.add_token(TokenType::Minus, String::from("-"), String::from("-")),
        '*' => lexer.add_token(TokenType::Times, String::from("*"), String::from("*")),
        '/' => lexer.add_token(TokenType::Divide, String::from("/"), String::from("/")),
        '%' => lexer.add_token(TokenType::Modulo, String::from("%"), String::from("%")),
        '=' => {
            if lexer.peek() == '=' {
                lexer.advance();
                lexer.add_token(
                    TokenType::LessThanOrEqual,
                    String::from("<="),
                    String::from("<="),
                );
            } else {
                lexer.add_token(TokenType::LessThan, String::from("<"), String::from("<"));
            }
        }
        '"' => string('"', lexer),
        '\'' => string('\'', lexer),
        '<' => {
            if lexer.peek() == '=' {
                lexer.advance();
                lexer.add_token(
                    TokenType::LessThanOrEqual,
                    String::from("<="),
                    String::from("<="),
                );
            } else {
                lexer.add_token(TokenType::LessThan, String::from("<"), String::from("<"));
            }
        }
        '>' => {
            if lexer.peek() == '=' {
                lexer.advance();
                lexer.add_token(
                    TokenType::GreaterThanOrEqual,
                    String::from(">="),
                    String::from(">="),
                );
            } else {
                lexer.add_token(TokenType::GreaterThan, String::from(">"), String::from(">"));
            }
        }
        ':' => lexer.add_token(TokenType::Colon, String::from(":"), String::from(":")),
        '.' => lexer.add_token(TokenType::Period, String::from("."), String::from(".")),
        '#' => {
            while lexer.peek() != '\n' && lexer.peek() != '\0' {
                lexer.advance();
            }
            lexer.line += 1;
        }
        '\n' => {
            lexer.line += 1;
        }
        ' ' => {
            return;
        }
        '\t' => {
            return;
        }
        _ => {
            if chr.is_alphabetic() {
                lexer.current -= 1;
                identifier(lexer);
            } else if chr.is_numeric() {
                lexer.current -= 1;
                number(lexer);
            } else {
                panic!("Unexpected character: {} at line {}", chr, lexer.line + 1);
            }
        }
    }
}

pub fn scan_tokens(lexer: &mut Lexer) -> &Vec<Token> {
    while lexer.current < lexer.source.len() {
        scan_token(lexer)
    }
    lexer.add_token(TokenType::Eof, String::new(), String::new());
    &lexer.tokens
}
