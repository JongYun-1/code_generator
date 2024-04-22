mod config;
mod roboto_code_generator;
mod desugar;

use std::env;
use std::path::Path;
use std::{fs::File, io};
use std::io::{BufRead, Write};
use config::Config;
use roboto_code_generator::RobotoProgram;
use crate::desugar::{Instruction, parse_roboto_program};

fn main() -> io::Result<()> {
    //let config = Config::new("./src/config.txt");
    //let roboto_program = RobotoProgram::new(config);
    //let program_code = roboto_program.generate();
    let args: Vec<String> = env::args().collect();
    let in_path = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);

    //let lines: Vec<String> = program.lines().map(String::from).collect();
    let file = File::open(in_path)?;
    let mut file_lines = io::BufReader::new(file).lines();
    let lines: Vec<String> =  file_lines.map(|line| line.unwrap()).collect();

    let result = parse_roboto_program(lines);
    match result {
        Ok(res) => {
            let mut file = match File::create(out_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                    return Err(e);
                }
            };
            for line in res {
                file.write_all(line.as_bytes())?;
                file.write_all(b"\n")?;

            }
            Ok(())
        },
        Err(e) => {
            eprintln!("{}", e);
            Ok(())
        }
    }

    
    

    // Attempt to write the program code to the file
    //match file.write_all(program_code.as_bytes()) {
    //    Ok(_) => println!("Program successfully written to file."),
    //    Err(e) => eprintln!("Error writing to file: {}", e),
    //}
}
