use std::error::Error;
use thiserror::Error;

#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
    Sub,
    Div,
    Mul,
}

const STACK_CAPACITY: usize = 1;

#[derive(Debug, Error)]
enum KvmError {
    #[error("Stack overflow error")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Division by zero")]
    DivisionByZero,
}

struct Kvm {
    stack: Vec<i32>,
    program: Vec<Instruction>,
}

impl Kvm {
    fn new() -> Self {
        Kvm {
            stack: Vec::with_capacity(STACK_CAPACITY),
            program: Vec::new(),
        }
    }

    fn execute_program(&mut self) -> Result<(), KvmError> {
        for inst in self.program.iter() {
            match inst {
                Instruction::Push(n) => {
                    if self.stack.len() >= STACK_CAPACITY {
                        return Err(KvmError::StackOverflow);
                    }
                    self.stack.push(*n);
                }
                Instruction::Add => {
                    let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    self.stack.push(n1 + n2);
                }
                Instruction::Sub => {
                    let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    self.stack.push(n1 - n2);
                }
                Instruction::Div => {
                    let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;

                    if n2 == 0 {
                        return Err(KvmError::DivisionByZero);
                    }

                    self.stack.push(n1 / n2);
                }
                Instruction::Mul => {
                    let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                    self.stack.push(n1 * n2);
                }
            }
        }

        Ok(())
    }

    fn load_program(&mut self, prog: Vec<Instruction>) {
        self.program.extend(prog);
    }

    fn dump_stack(&self) {
        println!("Stack: ");

        if self.stack.is_empty() {
            println!("[Empty]");
        } else {
            self.stack.iter().for_each(|e| println!("{}", e));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = Kvm::new();

    let prog = vec![Instruction::Push(1), Instruction::Push(3), Instruction::Add];

    vm.dump_stack();
    vm.load_program(prog);

    println!("------");

    vm.dump_stack();
    vm.execute_program()?;

    vm.dump_stack();

    Ok(())
}
