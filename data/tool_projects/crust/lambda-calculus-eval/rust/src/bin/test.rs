use lambda_calculus_eval::common;
use lambda_calculus_eval::config;
use lambda_calculus_eval::io;
use lambda_calculus_eval::parser; 
use lambda_calculus_eval::reducer;
use lambda_calculus_eval::typechecker;
use lambda_calculus_eval::hash_table::HashTable;
use lambda_calculus_eval::common::{AstNode, AstNodeType};
use std::io::{Read, Seek, Write};


#[test]
fn test_trim() {
let mut str_to_trim = String::from("  test  ");
config::trim(&mut str_to_trim);
assert_eq!(str_to_trim, "test");
}

#[test]
fn test_get_config_type() {
let file_opt = config::get_config_type("file");
let step_opt = config::get_config_type("step_by_step_reduction");
let reduction = config::get_config_type("reduction_order");
assert_eq!(file_opt, config::option_type_t::FILENAME);
assert_eq!(step_opt, config::option_type_t::STEP_REDUCTION);
assert_eq!(reduction, config::option_type_t::REDUCTION_ORDER);
}

#[test]
fn test_parse_config() {
let line = "file=expr.lambda";
let mut key = String::new();
let mut value = String::new();
config::parse_config(line, &mut key, &mut value);
assert_eq!(key, "file");
assert_eq!(value, "expr.lambda");
}

const TEMP_PATH: &str = "temp/test.txt";

#[test]
fn test_create_file() {
let _ = io::create_file(TEMP_PATH).unwrap();
assert!(std::fs::metadata(TEMP_PATH).is_ok());
std::fs::remove_file(TEMP_PATH).unwrap();
}

#[test]
fn test_get_file() {
let mut c = io::create_file(TEMP_PATH).unwrap();
let mut d = io::get_file(TEMP_PATH, "r+").unwrap();
let test_str = "Hello";
write!(c, "{}", test_str).unwrap();
c.flush().unwrap();
d.seek(std::io::SeekFrom::Start(0)).unwrap();
let mut buffer = String::new();
d.read_to_string(&mut buffer).unwrap();
assert_eq!(buffer, test_str);
io::delete_file(TEMP_PATH).unwrap();
}

#[test]
fn test_file_deletion() {
let _ = io::create_file(TEMP_PATH).unwrap();
assert!(std::fs::metadata(TEMP_PATH).is_ok());
io::delete_file(TEMP_PATH).unwrap();
assert!(std::fs::metadata(TEMP_PATH).is_err());
}

#[test]
fn test_parse_token() {
assert_eq!(parser::parse_token('('), common::tokens_t::L_PAREN);
assert_eq!(parser::parse_token(')'), common::tokens_t::R_PAREN);
assert_eq!(parser::parse_token('@'), common::tokens_t::LAMBDA);
assert_eq!(parser::parse_token('.'), common::tokens_t::DOT);
assert_eq!(parser::parse_token('t'), common::tokens_t::VARIABLE);
assert_eq!(parser::parse_token(' '), common::tokens_t::WHITESPACE);
assert_eq!(parser::parse_token('\n'), common::tokens_t::NEWLINE);
assert_eq!(parser::parse_token('='), common::tokens_t::EQ);
assert_eq!(parser::parse_token('"'), common::tokens_t::QUOTE);
assert_eq!(parser::parse_token(':'), common::tokens_t::COLON);
assert_eq!(parser::parse_token('$'), common::tokens_t::ERROR);
}

#[test]
fn test_is_variable() {
assert!(parser::is_variable('t'));
assert!(!parser::is_variable('*'));
}

#[test]
fn test_create_variable() {
let name = "test";
let type_ = "Nat";
let var = parser::create_variable(name, type_);
if let common::AstNodeUnion::Variable(variable) = var.node {
assert_eq!(var.type_, AstNodeType::VAR);
assert_eq!(variable.name, name);
assert_eq!(variable.type_, type_);
} else {
panic!("Expected variable node");
}
}

#[test]
fn test_create_application() {
let app = parser::create_application(&AstNode::default(), &AstNode::default());
if let common::AstNodeUnion::Application(application) = app.node {
assert_eq!(app.type_, AstNodeType::APPLICATION);
assert!(application.function.is_some());
assert!(application.argument.is_some());
} else {
panic!("Expected application node");
}
}

#[test]
fn test_create_lambda() {
let param = "test";
let type_ = "nat";
let lambda = parser::create_lambda(param, &AstNode::default(), type_);
if let common::AstNodeUnion::LambdaExpr(lambda_expr) = lambda.node {
assert_eq!(lambda.type_, AstNodeType::LAMBDA_EXPR);
assert_eq!(lambda_expr.parameter, param);
assert_eq!(lambda_expr.type_, type_);
assert!(lambda_expr.body.is_some());
} else {
panic!("Expected lambda expression node");
}
}

#[test]
fn test_alpha_convert() {
let old = "x";
let old_two = "y";
let expected = "x_1";
let expected_two = "y_2";
let new = parser::alpha_convert(old);
let new_two = parser::alpha_convert(old_two);
assert_eq!(new, expected);
assert_eq!(new_two, expected_two);
}

#[test]
fn test_is_used() {
let mut table = HashTable::new();
let key = "test";
table.insert(key, AstNode::default());
assert!(parser::is_used(&table, key));
assert!(!parser::is_used(&table, "wrong"));
}

#[test]
fn test_is_uppercase() {
assert!(parser::is_uppercase('A'));
assert!(!parser::is_uppercase('c'));
}

#[test]
fn test_expand_definitions() {
let mut table = HashTable::new();
let mut variable = parser::create_variable("test", "");
variable.type_ = AstNodeType::DEFINITION;
let definition = parser::create_variable("testing", "Nat");
table.insert("test", definition);
reducer::expand_definitions(&mut table, &mut variable);
if let common::AstNodeUnion::Variable(var) = variable.node {
assert_eq!(variable.type_, AstNodeType::VAR);
assert_eq!(var.name, "testing");
assert_eq!(var.type_, "Nat");
} else {
panic!("Expected variable node");
}
}

#[test]
fn test_replace() {
let mut variable = parser::create_variable("test", "");
let mut lambda_expr = parser::create_lambda("test", &variable, "Nat");
let new_name = "replaced";
reducer::replace(&mut lambda_expr, "test", new_name);
if let common::AstNodeUnion::LambdaExpr(lambda) = lambda_expr.node {
assert_eq!(lambda.parameter, new_name);
if let common::AstNodeUnion::Variable(var) = lambda.body.unwrap().node {
assert_eq!(var.name, new_name);
} else {
panic!("Expected variable node");
}
} else {
panic!("Expected lambda expression node");
}
}

#[test]
fn test_reduce_ast() {
let mut table = HashTable::new();
table.insert("Nat", AstNode::default());
let lambda_body = parser::create_variable("x", "");
let y_var = parser::create_variable("y", "Nat");
let lambda_expr = parser::create_lambda("x", &lambda_body, "Nat");
let parsed = parser::create_application(&lambda_expr, &y_var);
let reduced = reducer::reduce_ast(&mut table, &parsed);
if let common::AstNodeUnion::Variable(var) = reduced.node {
assert_eq!(reduced.type_, AstNodeType::VAR);
assert_eq!(var.name, "y");
assert_eq!(var.type_, "Nat");
} else {
panic!("Expected variable node");
}
}

#[test]
fn test_deepcopy() {
let lambda_body = parser::create_variable("x", "");
let lambda_expr = parser::create_lambda("x", &lambda_body, "Nat");
let deepcopied = reducer::deepcopy(&lambda_expr);
if let common::AstNodeUnion::LambdaExpr(lambda) = deepcopied.node {
assert_eq!(deepcopied.type_, AstNodeType::LAMBDA_EXPR);
assert_eq!(lambda.parameter, "x");
assert_eq!(lambda.type_, "Nat");
if let common::AstNodeUnion::Variable(var) = lambda.body.unwrap().node {
assert_eq!(var.name, "x");
} else {
panic!("Expected variable node");
}
} else {
panic!("Expected lambda expression node");
}
}

#[test]
fn test_type_equal() {
let a = typechecker::create_type("Nat", "", &AstNode::default());
let b = typechecker::create_type("Nat", "", &AstNode::default());
let c = typechecker::create_type("Bool", "", &AstNode::default());
assert!(typechecker::type_equal(&a, &b));
assert!(!typechecker::type_equal(&a, &c));
}

#[test]
fn test_get_type_from_expr() {
let n = parser::create_variable("test", "Nat");
let n1 = parser::create_lambda("test", &AstNode::default(), "Bool");
let n_type = typechecker::get_type_from_expr(&n);
let n1_type = typechecker::get_type_from_expr(&n1);
assert_eq!(n_type, "Nat");
assert_eq!(n1_type, "Bool");
}

#[test]
fn test_expr_type_equal() {
let n = parser::create_variable("test", "Nat");
let a = typechecker::create_type("Nat", "", &n);
assert!(typechecker::expr_type_equal(&a, &n));
}

#[test]
fn test_typecheck() {
let lambda_body = parser::create_variable("x", "");
let y_var = parser::create_variable("y", "Nat");
let lambda_expr = parser::create_lambda("x", &lambda_body, "Nat");
let parsed = parser::create_application(&lambda_expr, &y_var);
typechecker::typecheck(&parsed, None);
}

fn main() {
    
}
