// C. Wyatt Polasek + Zach Breene
// rUM Main Module
// rUM - main.rs

mod loading;
mod memory;
mod registers;
mod instructions;
mod execution;

use loading::load_um_program;
use execution::UMExecution;

/// Main entry point for the rUM (Universal Machine) emulator.
///
/// Processes command-line arguments to load and run a UM program.
///
/// Expects exactly one argument, which is the path to the UM program file.
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <UM-program-file>", args[0]);
        std::process::exit(1);
    }

    let um_program_path = &args[1];

    // Load the UM program
    let um_program = match load_um_program(um_program_path) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error loading UM program: {}", e);
            std::process::exit(1);
        },
    };

    // Initialize machine state
    let mut um = UMExecution::new();
    um.initialize(um_program);

    // Start execution
    let rum = um.run();

    // Handle any errors during execution
    if let Err(e) = rum {
        eprintln!("Error during execution: {:?}", e);
        std::process::exit(1);
    }
}