#[derive(Debug, Clone)]
pub enum Instruction {
    Halt,
    Add,
    Sub,
    Div,
    Mul,
    Eq,
    Push(i32),
    Jmp(u32),
    JmpIf(u32),
    Dup(u32),
    PushStr(String),
    PrintStack,
    PrintStr,
}
