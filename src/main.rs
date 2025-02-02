use std::{borrow::BorrowMut, fs, path::Path};
use bf_compiler::*;

fn main() {
    let tokens = lexer::generate_tokens(fs::File::open(Path::new("programs/hello_world.bf")).expect("File wont open lil guy").borrow_mut());

    let parser = parser::Parser(tokens);

    let ast = parser.parse();

    let mut asm = String::new();

    let mut loop_counter = 0;
    
    assembly::compile_ast(ast, &mut asm, &mut loop_counter);

    println!("{}", asm);
}
