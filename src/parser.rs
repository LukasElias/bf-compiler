use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug, Clone)]
struct LoopCounter {
    start: usize,
    loop_depth: usize,
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Pointer(isize),
    Value(isize),
    Loop(AbstractSyntaxTree),
    Input,
    Output,
    ProgramEnd,
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionType::Pointer(change) => { write!(f, "Pointer:\n\t{}\n", change).expect("Couldn't write to stdout"); },
            ExpressionType::Value(change) => { write!(f, "Value:\n\t{}\n", change).expect("Couldn't write to stdout"); },
            ExpressionType::Loop(tree) => { print!("Loop:\n\t{}", tree); },
            ExpressionType::Input => { write!(f, "Input\n").expect("Couldn't write to stdout"); },
            ExpressionType::Output => { write!(f, "Output\n").expect("Couldn't write to stdout"); },
            ExpressionType::ProgramEnd => { write!(f, "ProgramEnd\n").expect("Couldn't write to stdout"); },
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree(pub Vec<ExpressionType>);

impl Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for expression in &self.0 {
            write!(f, "{}", expression).expect("Couldn't write to stdout");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Parser(pub Vec<Token>);

impl Parser {
    pub fn parse(&self) -> AbstractSyntaxTree {
        let mut ast = AbstractSyntaxTree(vec![]);
        let mut pointer_token_counter = 0;
        let mut value_token_counter = 0;
        let mut loop_counter: Option<LoopCounter> = None;

        for (index, token) in self.0.iter().enumerate() {
            if loop_counter.is_some() && *token != Token::LoopEnd {
                if *token == Token::LoopStart {
                    loop_counter.clone().unwrap().loop_depth += 1;
                }

                continue;
            } else if pointer_token_counter != 0 && (*token != Token::PointerIncrease && *token != Token::PointerDecrease) {
                ast.0.push(ExpressionType::Pointer(pointer_token_counter));
                pointer_token_counter = 0;
            } else if value_token_counter != 0 && (*token != Token::ValueIncrease && *token != Token::ValueDecrease) {
                ast.0.push(ExpressionType::Value(value_token_counter));
                value_token_counter = 0;
            }

            match *token {
                Token::PointerIncrease => pointer_token_counter += 1,
                Token::PointerDecrease => pointer_token_counter -= 1,
                Token::ValueIncrease => value_token_counter += 1,
                Token::ValueDecrease => value_token_counter -= 1,
                Token::LoopStart => {
                    if loop_counter.is_some() {
                        panic!("Oh no");
                    }

                    loop_counter = Some(LoopCounter {
                        start: index,
                        loop_depth: 0,
                    });
                },
                Token::LoopEnd => {
                    if loop_counter.is_none() {
                        panic!("Loop is NOT started, {}", index);
                    } 

                    if loop_counter.clone().unwrap().loop_depth > 0 {
                        loop_counter.clone().unwrap().loop_depth -= 1;
                        continue
                    }

                    let parser = Parser(self.0[loop_counter.unwrap().start + 1..index].to_vec());

                    ast.0.push(ExpressionType::Loop(parser.parse()));

                    loop_counter = None;
                },
                Token::Input => ast.0.push(ExpressionType::Input),
                Token::Output => ast.0.push(ExpressionType::Output),
                Token::ProgramEnd => ast.0.push(ExpressionType::ProgramEnd),
            }
        }

        ast
    }
}
