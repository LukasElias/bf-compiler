use std::{borrow::BorrowMut, fs, path::Path};
use bf_compiler::*;

fn main() {
    let tokens = lexer::generate_tokens(fs::File::open(Path::new("programs/hello_world.bf")).expect("File wont open lil guy").borrow_mut());

    for token in tokens {
        println!("{:?}", token);
    }
}
