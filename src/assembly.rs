use crate::parser::{AbstractSyntaxTree, ExpressionType};

pub fn compile_ast(ast: AbstractSyntaxTree, asm: &mut String, loop_counter: &mut usize) {
    asm.clear();

    asm.push_str(".global main\nmain:\n    mov $30000, %rdi\n    call malloc\n    mov %rax, %rdi\n    mov $0, %rsi\n");

    for node in ast.0 {
        match node {
            ExpressionType::Pointer(value) => {
                if value >= 0 {
                    asm.push_str(format!("    add ${}, %rsi\n", value).as_str());
                } else if value <= 0 {
                    asm.push_str(format!("    sub ${}, %rsi\n", value.abs()).as_str());
                } else {
                    continue
                }
            },
            ExpressionType::Value(value) => {
                asm.push_str(format!("    addb ${}, (%rdi, %rsi, 1)\n", value).as_str());
                asm.push_str(format!("    subb ${}, (%rdi, %rsi, 1)\n", value).as_str());
            },
            ExpressionType::Loop(inner) => {
                let loop_start = *loop_counter;
                let loop_end = loop_start + 1;
                *loop_counter += 2;

                asm.push_str(&format!("L{}:\n", loop_start));
                asm.push_str("    cmpb $0, (%rdi, %rsi, 1)\n");
                asm.push_str(&format!("    je L{}\n", loop_end));

                compile_ast(inner, asm, loop_counter);

                if loop_start != 0 {
                    asm.push_str(&format!("    jmp L{}\n", loop_start));
                }

                asm.push_str(&format!("L{}:\n", loop_end));
            },
            ExpressionType::Output => asm.push_str(
                "    movzbq (%rdi, %rsi, 1), %rdi\n    call putchar\n",
            ),
            ExpressionType::Input=> asm.push_str(
                "    call getchar\n    movb %al, (%rdi, %rsi, 1)\n",
            ),
            ExpressionType::ProgramEnd => {
                asm.push_str("    mov $60, %rax\n    mov $0, %rdi\n    syscall");
            },
        }
    }
}
