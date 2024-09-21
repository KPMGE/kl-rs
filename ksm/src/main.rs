use kvm::{Instruction, Kvm};
use std::error::Error;

use clap::Parser;

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
        let prog_inst: Vec<Instruction> = prog_asm
            .lines()
            .map(|line| line.trim().try_into().unwrap())
            .collect();

        println!("prog: {:?}", prog_inst);
        // TODO: handle option without expect
        kvm::save_program_to_file(
            &prog_inst,
            &args.output_file.expect("Expected output file!"),
        );
    }

    Ok(())
}
