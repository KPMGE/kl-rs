pub mod instruction;
pub mod error;

use std::{fmt::Display, fs::File, io::Write};

use instruction::Instruction;

// TODO: find a better place for this function
pub fn save_program_to_file(program: Vec<Instruction>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    let prog_bin: Vec<u8> = program.iter().flat_map(|inst| inst.as_bytes()).collect();
    file.write_all(prog_bin.as_ref()).unwrap();
}

impl TryFrom<&str> for Instruction {
    // TODO: add better error
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "halt" => Ok(Instruction::Halt),
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "div" => Ok(Instruction::Div),
            "mul" => Ok(Instruction::Mul),
            "eq" => Ok(Instruction::Eq),
            _ if value.starts_with("push") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for push".to_string())?
                    .parse::<i32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::Push(n))
            }
            _ if value.starts_with("jmp") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for jmp".to_string())?
                    .parse::<u32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::Jmp(n))
            }
            _ if value.starts_with("jmpif") => {
                let n = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| "could not get argument for jmpif".to_string())?
                    .parse::<u32>()
                    .map_err(|e| format!("{:?}", e))?;

                Ok(Instruction::JmpIf(n))
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
        };
        write!(f, "{}", s)
    }
}

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
        };

        bytes
    }
}
