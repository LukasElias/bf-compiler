use std::{
    fs::File,
    io::Read,
};

#[derive(Debug)]
pub enum Token {
    PointerIncrease,
    PointerDecrease,
    ValueIncrease,
    ValueDecrease,
    LoopStart,
    LoopEnd,
    Input,
    Output,
}

pub fn generate_tokens(program_input: &mut File) -> Vec<Token> {
    let mut string = String::new();
    program_input.read_to_string(&mut string).expect("No string to write the file buffer to 🫡");

    let mut token_vec = Vec::<Token>::new();

    for char in string.chars() {
        let token: Token = match char {
            '>' => Token::PointerIncrease,
            '<' => Token::PointerDecrease,
            '+' => Token::ValueIncrease,
            '-' => Token::ValueDecrease,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            ',' => Token::Input,
            '.' => Token::Output,
            _ => { continue; },
        };

        token_vec.push(token);
    }

    token_vec
}
