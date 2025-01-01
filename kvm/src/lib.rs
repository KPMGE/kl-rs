pub mod error;
pub mod instruction;
pub mod kvm;

pub use error::*;
pub use instruction::*;
pub use kvm::*;

use std::fmt::Display;

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
            Instruction::PushStr(str) => format!("pushstr \"{}\"", str),
            Instruction::PrintStack => "printstack".to_string(),
            Instruction::PrintStr => "printstr".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Instruction {
    pub fn upcode(&self) -> u8 {
        match self {
            Instruction::Halt => 0x0,
            Instruction::Add => 0x1,
            Instruction::Sub => 0x2,
            Instruction::Div => 0x3,
            Instruction::Mul => 0x4,
            Instruction::Eq => 0x5,
            Instruction::Push(_) => 0x6,
            Instruction::Jmp(_) => 0x7,
            Instruction::JmpIf(_) => 0x8,
            Instruction::Dup(_) => 0x9,
            Instruction::PushStr(_) => 0x10,
            Instruction::PrintStack => 0x11,
            Instruction::PrintStr => 0x12,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Instruction::Halt => bytes.push(Instruction::Halt.upcode()),
            Instruction::Add => bytes.push(Instruction::Add.upcode()),
            Instruction::Sub => bytes.push(Instruction::Sub.upcode()),
            Instruction::Div => bytes.push(Instruction::Div.upcode()),
            Instruction::Mul => bytes.push(Instruction::Mul.upcode()),
            Instruction::Eq => bytes.push(Instruction::Eq.upcode()),
            Instruction::PrintStack => bytes.push(Instruction::PrintStack.upcode()),
            Instruction::PrintStr => bytes.push(Instruction::PrintStr.upcode()),
            Instruction::Push(n) => {
                bytes.push(Instruction::Push(0).upcode());
                bytes.extend(n.to_le_bytes());
            }
            Instruction::Jmp(n) => {
                bytes.push(Instruction::Jmp(0).upcode());
                bytes.extend(n.to_le_bytes());
            }
            Instruction::JmpIf(n) => {
                bytes.push(Instruction::JmpIf(0).upcode());
                bytes.extend(n.to_le_bytes());
            }
            Instruction::Dup(n) => {
                bytes.push(Instruction::Dup(0).upcode());
                bytes.extend(n.to_le_bytes());
            }
            Instruction::PushStr(str) => {
                bytes.push(Instruction::PushStr("".to_string()).upcode());
                bytes.extend(str.bytes());
            }
        };

        bytes
    }
}
