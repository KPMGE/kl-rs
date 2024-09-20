use kvm::Instruction;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: ksm <input.ksm> <output.kvm>");
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let prog_asm = std::fs::read_to_string(input_file)?;
    let prog: Vec<Instruction> = prog_asm
        .lines()
        .map(|line| line.trim().try_into().unwrap())
        .collect();

    println!("prog: {:?}", prog);

    kvm::save_program_to_file(prog, output_file);

    Ok(())
}
