use std::fs::File;
use std::io::Read;

use crate::{error::KvmError, instruction::Instruction};

const STACK_CAPACITY: usize = 1024;
const MAX_INSTRUCTIONS: usize = 1000;

pub struct Kvm {
    stack: Vec<i32>,
    program: Vec<Instruction>,
    string_pool: Vec<String>,
    ip: usize,
    halt: bool,
}

impl Kvm {
    pub fn new() -> Self {
        Kvm {
            stack: Vec::with_capacity(STACK_CAPACITY),
            program: Vec::new(),
            string_pool: Vec::new(),
            ip: 0,
            halt: false,
        }
    }

    pub fn execute_program(&mut self) -> Result<(), KvmError> {
        for _ in 0..MAX_INSTRUCTIONS {
            if self.halt {
                break;
            }

            let inst = self.program.get(self.ip).unwrap();
            self.execute_instruction(inst.clone())?;
        }

        Ok(())
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.program
    }

    pub fn load_program_from_vec(&mut self, prog: Vec<Instruction>) {
        self.program.extend(prog);
    }

    fn extract_string_from_bytes(&self, bytes: &[u8]) -> Option<String> {
        if let Ok(text) = std::str::from_utf8(bytes) {
            if let (Some(start), Some(end)) = (text.find('"'), text.rfind('"')) {
                if start < end {
                    return Some(text[start + 1..end].to_string());
                }
            }
        }
        None
    }

    pub fn load_program_from_file(&mut self, file_path: &str) {
        let mut file = File::open(file_path).unwrap();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut instructions = Vec::new();
        let mut i = 0;

        // TODO: find a better way to map byte code to instructions
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
                    // TODO: find a better way to construct this
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
                    // TODO: find a better way to construct this
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
                    // TODO: find a better way to construct this
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
                    // TODO: find a better way to construct this
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
                0x10 => {
                    let str = self
                        .extract_string_from_bytes(&buffer)
                        .expect("Could not extract string from bytes");

                    // skip "" and the string
                    i += str.len() + 2;

                    Instruction::PushStr(str.to_string())
                }
                0x11 => Instruction::PrintStack,
                0x12 => Instruction::PrintStr,
                upcode => panic!("Unknown instruction upcode {}", upcode),
            };

            instructions.push(inst);
            i += 1;
        }

        self.load_program_from_vec(instructions);
    }

    pub fn dump_stack(&self) {
        println!("Stack: ");

        if self.stack.is_empty() {
            println!("[Empty]");
        } else {
            self.stack.iter().for_each(|e| println!("{}", e));
        }
    }

    pub fn dump_program(&self) {
        println!("Program: ");
        self.program.iter().for_each(|inst| println!("{:?}", inst));
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
            Instruction::PushStr(s) => {
                self.string_pool.push(s);
                self.ip += 1;
            }
            Instruction::PrintStack => {
                println!(
                    "{}",
                    match self.stack.pop() {
                        Some(i) => i.to_string(),
                        None => "Empty".to_string(),
                    }
                );
                self.ip += 1;
            }
            Instruction::PrintStr => {
                println!(
                    "{}",
                    match self.string_pool.pop() {
                        Some(i) => i.to_string(),
                        None => "Empty".to_string(),
                    }
                );
                self.ip += 1;
            }
        };

        Ok(())
    }
}
