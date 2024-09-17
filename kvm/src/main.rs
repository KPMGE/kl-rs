use std::error::Error;
use thiserror::Error;

#[derive(Debug, Clone)]
enum Instruction {
    Add,
    Sub,
    Div,
    Mul,
    Push(i32),
    Jmp(usize),
    Dup(usize),
}

const STACK_CAPACITY: usize = 1024;

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
    ip: usize,
}
impl Kvm {
    fn new() -> Self {
        Kvm {
            stack: Vec::with_capacity(STACK_CAPACITY),
            program: Vec::new(),
            ip: 0,
        }
    }

    fn execute_instruction(&mut self, inst: Instruction) -> Result<(), KvmError> {
        match inst {
            Instruction::Push(n) => {
                if self.stack.len() >= STACK_CAPACITY {
                    return Err(KvmError::StackOverflow);
                }
                self.stack.push(n);
                self.ip += 1;
            }
            Instruction::Add => {
                let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                self.stack.push(n1 + n2);
                self.ip += 1;
            }
            Instruction::Sub => {
                let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                self.stack.push(n1 - n2);
                self.ip += 1;
            }
            Instruction::Div => {
                let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;

                if n2 == 0 {
                    return Err(KvmError::DivisionByZero);
                }

                self.stack.push(n1 / n2);
                self.ip += 1;
            }
            Instruction::Mul => {
                let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                self.stack.push(n1 * n2);
                self.ip += 1;
            }
            Instruction::Jmp(addr) => {
                self.ip = addr;
            }
            Instruction::Dup(addr) => {
                if self.stack.len() >= STACK_CAPACITY {
                    return Err(KvmError::StackOverflow);
                }

                let idx = self.stack.len() - addr;
                if idx <= 0 {
                    return Err(KvmError::StackUnderflow);
                }

                let elem = self.stack.get(idx - 1).unwrap();
                self.stack.push(*elem);
                self.ip += 1;
            }
        };

        Ok(())
    }

    fn execute_program(&mut self) -> Result<(), KvmError> {
        let n = 100;
        for _ in 0..n {
            let inst = self.program.get(self.ip).unwrap();
            println!("Executing: {:?}", inst);
            self.execute_instruction(inst.clone())?;
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

    let prog = vec![
        Instruction::Push(0),
        Instruction::Push(1),
        Instruction::Dup(1),
        Instruction::Dup(1),
        Instruction::Add,
        Instruction::Jmp(2),
    ];

    vm.load_program(prog);
    vm.execute_program()?;
    vm.dump_stack();

    Ok(())
}
