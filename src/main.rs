mod config;
mod roboto_code_generator;
mod desugar;

use std::fs::File;
use std::io::Write;
use config::Config;
use roboto_code_generator::RobotoProgram;
use crate::desugar::{Instruction, parse_roboto_program};

fn main() {
    let config = Config::new("./src/config.txt");
    let roboto_program = RobotoProgram::new(config);
    let program_code = roboto_program.generate();

    let program = r#"
label 0:
    Flip 2 1 10
    Turn Left 0
    Turn Right 0
label 100:
    Sense Ahead 101 0 Food
    PickUp 200 0
label 200:
    Sense Ahead 201 0 HomeBase
    Move 200 0
    Drop 0
"#;
    let labels = parse_roboto_program(program);

    for label in labels {
        println!("label {}:", label.number);
        for instruction in label.instructions {
            match instruction {
                Instruction::Flip { p, l1, l2 } => println!("    Flip {} {} {}", p, l1, l2),
                Instruction::Turn { lr, l } => println!("    Turn {} {}", lr, l),
                Instruction::Sense { sensedir, l1, l2, cond } => println!("    Sense {} {} {} {}", sensedir, l1, l2, cond),
                Instruction::PickUp { l1, l2 } => println!("    PickUp {} {}", l1, l2),
                Instruction::Move { l1, l2 } => println!("    Move {} {}", l1, l2),
                Instruction::Drop { l } => println!("    Drop {}", l),
                _ => {}
            }
        }
    }
    let mut file = match File::create("collecting.roboto") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    // Attempt to write the program code to the file
    match file.write_all(program_code.as_bytes()) {
        Ok(_) => println!("Program successfully written to file."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}
