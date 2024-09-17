use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};
use thiserror::Error;

#[derive(Debug, Clone)]
enum Instruction {
    Halt,
    Add,
    Sub,
    Div,
    Mul,
    Push(i32),
    Jmp(usize),
    Dup(usize),
}

impl Instruction {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match *self {
            Instruction::Halt => bytes.push(0x0),
            Instruction::Add => bytes.push(0x1),
            Instruction::Sub => bytes.push(0x2),
            Instruction::Div => bytes.push(0x3),
            Instruction::Mul => bytes.push(0x4),
            Instruction::Push(n) => bytes.extend(vec![0x5, n.to_le_bytes()[0]]),
            Instruction::Jmp(n) => bytes.extend(vec![0x6, n.to_le_bytes()[0]]),
            Instruction::Dup(n) => bytes.extend(vec![0x7, n.to_le_bytes()[0]]),
        };

        bytes
    }
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
    halt: bool,
}
impl Kvm {
    fn new() -> Self {
        Kvm {
            stack: Vec::with_capacity(STACK_CAPACITY),
            program: Vec::new(),
            ip: 0,
            halt: false,
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
            Instruction::Halt => self.halt = true,
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

    fn load_program_from_vec(&mut self, prog: Vec<Instruction>) {
        self.program.extend(prog);
    }

    fn save_program_to_file(&self, file_path: &str) {
        let mut file = File::create(file_path).unwrap();
        let prog_bin: Vec<u8> = self
            .program
            .iter()
            .flat_map(|inst| inst.as_bytes())
            .collect();
        file.write_all(prog_bin.as_ref()).unwrap();
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

    vm.load_program_from_vec(prog);
    vm.save_program_to_file("test.kvm");
    vm.execute_program()?;
    vm.dump_stack();

    Ok(())
}
