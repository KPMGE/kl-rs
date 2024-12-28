pub mod error;
pub mod instruction;
pub mod kvm;

pub use error::*;
pub use instruction::*;
pub use kvm::*;

use std::fmt::Display;

impl TryFrom<&str> for Instruction {
    // TODO: add better error
    // TODO: refactor this implementation
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "halt" => Ok(Instruction::Halt),
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "div" => Ok(Instruction::Div),
            "mul" => Ok(Instruction::Mul),
            "eq" => Ok(Instruction::Eq),
            "print_str" => Ok(Instruction::PrintStr),
            "print_stack" => Ok(Instruction::PrintStack),
            _ if value.starts_with("push_str") => {
                let str = value
                    .split_once(' ')
                    .ok_or_else(|| "could not get argument for push_str".to_string())?
                    .1;

                println!("PUSHING STRING: {str}");

                Ok(Instruction::PushStr(str.to_string()))
            }
            // TODO: get rid of these starts_with
            _ if value.starts_with("push") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for push".to_string())?
                    .parse::<i32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::Push(n))
            }
            _ if value.starts_with("jmpif") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for jmp".to_string())?
                    .parse::<u32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::JmpIf(n))
            }
            _ if value.starts_with("jmp") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for jmpif".to_string())?
                    .parse::<u32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::Jmp(n))
            }
            _ if value.starts_with("dup") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for dup".to_string())?
                    .parse::<u32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::Dup(n))
            }
            _ => {
                panic!("Invalid instruction: {}", value);
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Instruction::Halt => "halt".to_string(),
            Instruction::Add => "add".to_string(),
            Instruction::Sub => "sub".to_string(),
            Instruction::Div => "div".to_string(),
            Instruction::Mul => "mul".to_string(),
            Instruction::Eq => "eq".to_string(),
            Instruction::Push(n) => format!("push {}", n),
            Instruction::Jmp(addr) => format!("jmp {}", addr),
            Instruction::JmpIf(addr) => format!("jmpif {}", addr),
            Instruction::Dup(addr) => format!("dup {}", addr),
            Instruction::PushStr(str) => format!("push_str {}", str),
            Instruction::PrintStack => "print_stack".to_string(),
            Instruction::PrintStr => "print_str".to_string(),
        };
        write!(f, "{}", s)
    }
}

// TODO: find a better way to convert these to byte code
impl Instruction {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Instruction::Halt => bytes.push(0x0),
            Instruction::Add => bytes.push(0x1),
            Instruction::Sub => bytes.push(0x2),
            Instruction::Div => bytes.push(0x3),
            Instruction::Mul => bytes.push(0x4),
            Instruction::Eq => bytes.push(0x5),
            Instruction::Push(n) => {
                bytes.push(0x6);
                bytes.extend(n.to_le_bytes());
            }
            Instruction::Jmp(n) => {
                bytes.push(0x7);
                bytes.extend(n.to_le_bytes());
            }
            Instruction::JmpIf(n) => {
                bytes.push(0x8);
                bytes.extend(n.to_le_bytes());
            }
            Instruction::Dup(n) => {
                bytes.push(0x9);
                bytes.extend(n.to_le_bytes());
            }
            Instruction::PushStr(str) => {
                bytes.push(0x10);
                bytes.extend(str.bytes());
            }
            Instruction::PrintStack => {
                bytes.push(0x11);
            }
            Instruction::PrintStr => {
                bytes.push(0x12);
            }
        };

        bytes
    }
}
