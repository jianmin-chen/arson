#![macro_use]
use std::collections::HashMap;

#[derive(Debug)]
pub enum AstType {
    Var,
    Func,
    Class,
    Array,
    Dict,
    Number,
    String,
    Bool,
    BinOp,
    Return,
    If,
    Else,
    While,
    For,
    Attr,
    Call,
    Chain,
}

#[derive(Clone, Debug)]
pub enum Number {
    Int(isize),
    Float(f64),
}

#[derive(Debug)]
pub enum AstValues {
    Type(AstType),
    String(String),
    Strings(Vec<String>),
    Number(Number),
    Bool(bool),
    Node(Node),
    Nodes(Vec<Node>),
    Dict(HashMap<String, AstValues>),
}

pub type Node = HashMap<&'static str, AstValues>;

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("Mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

pub fn new_var(name: String, value: Option<AstValues>) -> Node {
    let value = value.unwrap_or(AstValues::String(String::new()));
    let mut var_ast = HashMap::new();
    var_ast.insert("kind", AstValues::Type(AstType::Var));
    var_ast.insert("name", AstValues::String(name));
    var_ast.insert("value", value);
    var_ast
}

pub fn new_func(name: String, args: Vec<String>, body: Vec<Node>) -> Node {
    let mut func_ast = HashMap::new();
    func_ast.insert("kind", AstValues::Type(AstType::Func));
    func_ast.insert("name", AstValues::String(name));
    func_ast.insert("args", AstValues::Strings(args));
    func_ast.insert("body", AstValues::Nodes(body));
    func_ast
}

pub fn new_class(name: String, methods: Vec<Node>) -> Node {
    let mut class_ast = HashMap::new();
    class_ast.insert("kind", AstValues::Type(AstType::Class));
    class_ast.insert("name", AstValues::String(name));
    class_ast.insert("methods", AstValues::Nodes(methods));
    class_ast
}

pub fn new_dict(items: HashMap<String, AstValues>) -> Node {
    let mut dict_ast = HashMap::new();
    dict_ast.insert("kind", AstValues::Type(AstType::Dict));
    dict_ast.insert("items", AstValues::Dict(items));
    dict_ast
}

pub fn new_array(items: Vec<Node>) -> Node {
    let mut array_ast = HashMap::new();
    array_ast.insert("kind", AstValues::Type(AstType::Array));
    array_ast.insert("value", AstValues::Nodes(items));
    array_ast
}

pub fn new_return(val: Node) -> Node {
    let mut return_ast = HashMap::new();
    return_ast.insert("kind", AstValues::Type(AstType::Return));
    return_ast.insert("value", AstValues::Node(val));
    return_ast
}

pub fn new_if(condition: Node, body: Vec<Node>, otherwise: Option<Vec<Node>>) -> Node {
    let otherwise = otherwise.unwrap_or(Vec::new());
    let mut if_ast = HashMap::new();
    if_ast.insert("kind", AstValues::Type(AstType::If));
    if_ast.insert("condition", AstValues::Node(condition));
    if_ast.insert("body", AstValues::Nodes(body));
    if_ast.insert("otherwise", AstValues::Nodes(otherwise));
    if_ast
}

pub fn new_else(body: Vec<Node>) -> Node {
    let mut else_ast = HashMap::new();
    else_ast.insert("kind", AstValues::Type(AstType::Else));
    else_ast.insert("body", AstValues::Nodes(body));
    else_ast
}

pub fn new_while(condition: Node, body: Vec<Node>) -> Node {
    let mut while_ast = HashMap::new();
    while_ast.insert("kind", AstValues::Type(AstType::While));
    while_ast.insert("condition", AstValues::Node(condition));
    while_ast.insert("body", AstValues::Nodes(body));
    while_ast
}

pub fn new_for(var: String, through: Vec<Node>, body: Vec<Node>) -> Node {
    let mut for_ast = HashMap::new();
    for_ast.insert("kind", AstValues::Type(AstType::For));
    for_ast.insert("var", AstValues::String(var));
    for_ast.insert("through", AstValues::Nodes(through));
    for_ast.insert("body", AstValues::Nodes(body));
    for_ast
}

pub fn new_number(val: AstValues) -> Node {
    let mut number_ast = HashMap::new();
    number_ast.insert("kind", AstValues::Type(AstType::Number));
    number_ast.insert("value", val);
    number_ast
}

pub fn new_string(val: AstValues) -> Node {
    let mut string_ast = HashMap::new();
    string_ast.insert("kind", AstValues::Type(AstType::String));
    string_ast.insert("value", val);
    string_ast
}

pub fn new_bool(val: AstValues) -> Node {
    let mut bool_ast = HashMap::new();
    bool_ast.insert("kind", AstValues::Type(AstType::Bool));
    bool_ast.insert("value", val);
    bool_ast
}

pub fn new_binop(left: Node, right: Node, op: String, wrapped: Option<bool>) -> Node {
    let wrapped = wrapped.unwrap_or(false);
    let mut binop_ast = HashMap::new();
    binop_ast.insert("kind", AstValues::Type(AstType::BinOp));
    binop_ast.insert("left", AstValues::Node(left));
    binop_ast.insert("right", AstValues::Node(right));
    binop_ast.insert("op", AstValues::String(op));
    binop_ast.insert("wrapped", AstValues::Bool(wrapped));
    binop_ast
}

pub fn new_call(args: Vec<Node>) -> Node {
    let mut call_ast = HashMap::new();
    call_ast.insert("kind", AstValues::Type(AstType::Call));
    call_ast.insert("value", AstValues::Nodes(args));
    call_ast
}

pub fn new_attr(attr: String) -> Node {
    let mut attr_ast = HashMap::new();
    attr_ast.insert("kind", AstValues::Type(AstType::Attr));
    attr_ast.insert("attr", AstValues::String(attr));
    attr_ast
}

pub fn new_chain(name: Node, chain: Vec<Node>) -> Node {
    let mut chain_ast = HashMap::new();
    chain_ast.insert("kind", AstValues::Type(AstType::Chain));
    chain_ast.insert("name", AstValues::Node(name));
    chain_ast.insert("chain", AstValues::Nodes(chain));
    chain_ast
}
