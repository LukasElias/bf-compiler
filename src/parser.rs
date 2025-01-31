use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Pointer(i32),
    Value(i32),
    Loop(AbstractSyntaxTree),
    Input,
    Output,
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionType::Pointer(change) => { write!(f, "Pointer:\n\t{}\n", change).expect("Couldn't write to stdout"); },
            ExpressionType::Value(change) => { write!(f, "Value:\n\t{}\n", change).expect("Couldn't write to stdout"); },
            ExpressionType::Loop(tree) => { write!(f, "Loop:\n\t{}\n", tree).expect("Couldn't write to stdout"); },
            ExpressionType::Input => { write!(f, "Input\n").expect("Couldn't write to stdout"); },
            ExpressionType::Output => { write!(f, "Output\n").expect("Couldn't write to stdout"); },
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree(Vec<ExpressionType>);

impl Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for expression in &self.0 {
            write!(f, "{}", expression).expect("Couldn't write to stdout");
        }

        Ok(())
    }
}

struct Parser {
    tokens: Vec<Token>
}
