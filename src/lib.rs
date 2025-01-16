pub mod callable;
pub mod class;
pub mod environment;
pub mod exceptions;
pub mod expr;
pub mod instance;
pub mod interpreter;
pub mod logger;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod stmt;
pub mod tokens;
pub mod utils;

use crate::resolver::Resolver;

pub fn run(source: &str) {
    let mut obj = scanner::Scanner::new(source);
    let result = obj.scan_tokens();
    // dbg!(&result);
    let mut pars = parser::Parser::new(result);
    let statements = pars.parse().unwrap();
    // dbg!(&statements);
    let mut intr = interpreter::Interpreter::new();
    let mut resolver = Resolver::new(&mut intr);
    resolver.resolve_statements(statements.clone());
    intr.interpret(statements);
}
