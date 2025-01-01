use kvm::{Instruction, Kvm};
use std::{error::Error, fs::File, io::Write};

use clap::Parser;

mod lexer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_file: Option<String>,

    #[arg(short, long)]
    disassemble: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.disassemble {
        let mut vm = Kvm::new();
        vm.load_program_from_file(&args.input_file);
        let instructions = vm.get_instructions();
        instructions.iter().for_each(|inst| println!("{}", inst));
    } else {
        let prog_asm = std::fs::read_to_string(args.input_file)?;

        let lexer = lexer::Lexer::new(&prog_asm);
        let prog_inst: Vec<Instruction> = lexer.collect();

        // TODO: handle option without expect
        save_program_to_file(
            &prog_inst,
            &args.output_file.expect("Expected output file!"),
        );
    }

    Ok(())
}

fn save_program_to_file(program: &[Instruction], file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    let prog_bin: Vec<u8> = program.iter().flat_map(|inst| inst.as_bytes()).collect();
    file.write_all(prog_bin.as_ref()).unwrap();
}
