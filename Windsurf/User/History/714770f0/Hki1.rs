use std::env;
use std::process;

mod lexer;
mod model;
mod parser;
mod exporter;

use parser::VdslParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.vdsl> <output.obj>", args[0]);
        process::exit(1);
    }
    
    let mut parser = VdslParser::new();
    
    if let Err(e) = parser.parse_and_export(&args[1], &args[2]) {
        eprintln!("Error parsing file: {}", e);
        process::exit(1);
    }
}
