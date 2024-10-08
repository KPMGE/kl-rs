use kvm::Instruction;
use std::{error::Error, fs::File, io::Read};
use thiserror::Error;

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
                self.ip = addr as usize;
            }
            Instruction::Halt => self.halt = true,
            Instruction::Dup(addr) => {
                if self.stack.len() >= STACK_CAPACITY {
                    return Err(KvmError::StackOverflow);
                }

                let idx = self.stack.len() - addr as usize;
                if idx <= 0 {
                    return Err(KvmError::StackUnderflow);
                }

                let elem = self.stack.get(idx - 1).unwrap();
                self.stack.push(*elem);
                self.ip += 1;
            }
            Instruction::Eq => {
                let n1 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                let n2 = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;
                self.stack.push((n1 == n2) as i32);
                self.ip += 1;
            }
            Instruction::JmpIf(addr) => {
                let n = self.stack.pop().ok_or_else(|| KvmError::StackUnderflow)?;

                if n > 0 {
                    self.ip = addr as usize;
                } else {
                    self.ip += 1;
                }
            }
        };

        Ok(())
    }

    fn execute_program(&mut self) -> Result<(), KvmError> {
        let n = 100;
        for _ in 0..n {
            if self.halt {
                println!("HALT");
                break;
            }

            let inst = self.program.get(self.ip).unwrap();
            self.execute_instruction(inst.clone())?;
        }

        Ok(())
    }

    fn load_program_from_vec(&mut self, prog: Vec<Instruction>) {
        self.program.extend(prog);
    }

    fn load_program_from_file(&mut self, file_path: &str) {
        let mut file = File::open(file_path).unwrap();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut instructions = Vec::new();
        let mut i = 0;

        while i < buffer.len() {
            let byte = buffer[i];

            let inst = match byte {
                0x0 => Instruction::Halt,
                0x1 => Instruction::Add,
                0x2 => Instruction::Sub,
                0x3 => Instruction::Div,
                0x4 => Instruction::Mul,
                0x5 => Instruction::Eq,
                0x6 => {
                    let num = i32::from_le_bytes([
                        buffer[i + 1],
                        buffer[i + 2],
                        buffer[i + 3],
                        buffer[i + 4],
                    ]);
                    // skip 4 bytes
                    i += 4;
                    Instruction::Push(num)
                }
                0x7 => {
                    let addr = u32::from_le_bytes([
                        buffer[i + 1],
                        buffer[i + 2],
                        buffer[i + 3],
                        buffer[i + 4],
                    ]);
                    // skip 4 bytes
                    i += 4;

                    Instruction::Jmp(addr)
                }
                0x8 => {
                    let addr = u32::from_le_bytes([
                        buffer[i + 1],
                        buffer[i + 2],
                        buffer[i + 3],
                        buffer[i + 4],
                    ]);
                    // skip 4 bytes
                    i += 4;

                    Instruction::JmpIf(addr)
                }
                0x9 => {
                    let addr = u32::from_le_bytes([
                        buffer[i + 1],
                        buffer[i + 2],
                        buffer[i + 3],
                        buffer[i + 4],
                    ]);
                    // skip 4 bytes
                    i += 4;
                    Instruction::Dup(addr)
                }
                _ => panic!("Invalid instruction!"),
            };

            instructions.push(inst);
            i += 1;
        }

        self.load_program_from_vec(instructions);
    }

    fn dump_stack(&self) {
        println!("Stack: ");

        if self.stack.is_empty() {
            println!("[Empty]");
        } else {
            self.stack.iter().for_each(|e| println!("{}", e));
        }
    }

    fn dump_program(&self) {
        println!("Program: ");
        self.program.iter().for_each(|inst| println!("{:?}", inst));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Usage: kvm <input.kvm>");
    }

    let mut vm = Kvm::new();

    vm.load_program_from_file(&args[1]);
    vm.dump_program();
    vm.execute_program()?;
    println!("--------");
    vm.dump_stack();

    Ok(())
}
