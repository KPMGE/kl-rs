#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
}

const STACK_CAPACITY: usize = 1024;

fn main() {
    let mut prog_stack: Vec<i32> = Vec::with_capacity(STACK_CAPACITY);

    let prog = vec![Instruction::Push(1), Instruction::Push(3), Instruction::Add];

    for inst in &prog {
        println!("{:#?}", inst);
    }

    for inst in &prog {
        match inst {
            Instruction::Push(n) => prog_stack.push(*n),
            Instruction::Add => {
                let n1 = prog_stack.pop().unwrap();
                let n2 = prog_stack.pop().unwrap();
                prog_stack.push(n1 + n2);
            }
        }
    }

    for e in &prog_stack {
        println!("{:#?}", e);
    }

    println!("Hello, world!");
}
