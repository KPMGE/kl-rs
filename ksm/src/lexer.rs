use kvm::Instruction;

#[derive(Debug)]
pub struct Lexer<'l> {
    input: &'l str,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l str) -> Lexer<'_> {
        Lexer {
            input,
            current_position: 0,
            read_position: 1,
            current_char: input.chars().nth(0),
        }
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        if self.current_char.unwrap() != '"' {
            panic!(
                "Unexpected start of string, expected: '\"', got: {:?}",
                self.current_char
            );
        }

        let mut str = String::new();
        self.read_char();

        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            str.push(c);
            self.read_char();
        }

        self.read_char();

        // TODO: handle strings without relying on ""
        str.push('"');
        str.insert(0, '"');

        str
    }

    fn read_number(&mut self) -> i32 {
        self.skip_whitespaces();
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.read_char();
                continue;
            }
            break;
        }

        self.input[start_pos..self.current_position]
            .parse::<i32>()
            .unwrap()
    }

    fn peek_char(&self, pos: usize) -> Option<char> {
        self.input.chars().nth(pos)
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.current_char = None;
            return;
        }

        let ch = self.input.chars().nth(self.read_position);

        self.current_char = ch;
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
                continue;
            }
            break;
        }
    }

    fn skip_comments(&mut self) {
        while let (Some(ch), Some(next_ch)) =
            (self.current_char, self.peek_char(self.read_position))
        {
            if ch == '*' && next_ch == '/' {
                break;
            }
            self.read_char();
        }
        self.read_char();
        self.read_char();
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_letter() {
                self.read_char();
                continue;
            }
            break;
        }

        let identifier = &self.input[start_pos..self.current_position];
        identifier.to_string()
    }
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        self.is_ascii_lowercase() || self.is_ascii_uppercase()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        let ch = self.current_char?;

        let inst: Option<Instruction> = match ch {
            '/' => {
                if self.peek_char(self.read_position).unwrap() == '*' {
                    self.skip_comments();
                }
                return self.next()
            },
            c => {
                if c.is_letter() {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "halt" => Some(Instruction::Halt),
                        "add" => Some(Instruction::Add),
                        "sub" => Some(Instruction::Sub),
                        "div" => Some(Instruction::Div),
                        "mul" => Some(Instruction::Mul),
                        "eq" => Some(Instruction::Eq),
                        "printstr" => Some(Instruction::PrintStr),
                        "printstack" => Some(Instruction::PrintStack),
                        "pushstr" => Some(Instruction::PushStr(self.read_string())),
                        "push" => Some(Instruction::Push(self.read_number())),
                        "jmpif" => Some(Instruction::JmpIf(self.read_number() as u32)),
                        "jmp" => Some(Instruction::Jmp(self.read_number() as u32)),
                        "dup" => Some(Instruction::Dup(self.read_number() as u32)),
                        _ => {
                            panic!("Invalid instruction: {}", identifier);
                        }
                    }
                } else {
                    panic!("Invalid instruction: {}", c);
                }
            }
        };

        self.read_char();
        inst
    }
}
