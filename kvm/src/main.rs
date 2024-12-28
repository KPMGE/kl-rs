use std::error::Error;

use kvm::Kvm;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Usage: kvm <input.kvm>");
    }

    let mut vm = Kvm::new();

    vm.load_program_from_file(&args[1]);
    // vm.dump_program();
    vm.execute_program()?;
    // println!("--------");
    // vm.dump_stack();

    Ok(())
}
