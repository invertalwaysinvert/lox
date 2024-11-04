use crate::tokens::{Token, TokenType};

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error_token(token: Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, "  at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {line}] Error{location}: {message}");
}
