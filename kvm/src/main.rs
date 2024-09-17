#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
}

const STACK_CAPACITY: usize = 1024;

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

    fn execute_program(&mut self) {
        for inst in self.program.iter() {
            match inst {
                Instruction::Push(n) => {
                    self.stack.push(*n);
                }
                Instruction::Add => {
                    let n1 = self.stack.pop().unwrap();
                    let n2 = self.stack.pop().unwrap();
                    self.stack.push(n1 + n2);
                }
            }
        }
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


fn main() {
    let mut vm = Kvm::new();

    let prog = vec![Instruction::Push(1), Instruction::Push(3), Instruction::Add];

    vm.dump_stack();
    vm.load_program(prog);

    println!("------");

    vm.dump_stack();
    vm.execute_program();

    println!("------");

    vm.dump_stack();
}
