use std::collections::HashMap;

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

pub enum AstValues {
    Type(AstType),
    String(String),
    VecString(Vec<String>),
    Number(isize),
    Bool(bool),
}

pub fn new_var(name: String, value: Option<String>) -> HashMap<&'static str, AstValues> {
    let value = value.unwrap_or(String::new());
    let mut var_ast = HashMap::new();
    var_ast.insert("kind", AstValues::Type(AstType::Var));
    var_ast.insert("name", AstValues::String(name));
    var_ast.insert("value", AstValues::String(value));
    var_ast
}

pub fn new_func(name: String, args: Vec<String>) -> HashMap<&'static str, AstValues> {
    let mut func_ast = HashMap::new();
    func_ast.insert("kind", AstValues::Type(AstType::Func));
    func_ast.insert("name", AstValues::String(name));
    func_ast.insert("args", AstValues::VecString(args));
    func_ast
}

pub fn new_class() {}

pub fn new_dict() {}

pub fn new_array() {}

pub fn new_return() {}

pub fn new_if() {}

pub fn new_else() {}

pub fn new_while() {}

pub fn new_for() {}

pub fn new_number(val: AstValues) -> HashMap<&'static str, AstValues> {
    let mut number_ast = HashMap::new();
    number_ast.insert("kind", AstValues::Type(AstType::Number));
    number_ast.insert("value", val);
    number_ast
}

pub fn new_string(val: AstValues) -> HashMap<&'static str, AstValues> {
    let mut string_ast = HashMap::new();
    string_ast.insert("kind", AstValues::Type(AstType::String));
    string_ast.insert("value", val);
    string_ast
}

pub fn new_bool(val: AstValues) -> HashMap<&'static str, AstValues> {
    let mut bool_ast = HashMap::new();
    bool_ast.insert("kind", AstValues::Type(AstType::Bool));
    bool_ast.insert("value", val);
    bool_ast
}

pub fn new_binop() {}

pub fn new_call() {}

pub fn new_attr() {}

pub fn new_chain() {}
