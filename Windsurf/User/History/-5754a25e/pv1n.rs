mod tokenizer;
mod vdl_generator;
mod neural_network;
mod model;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("usage: {} <command> [input] [output]", args[0]);
        eprintln!("commands:");
        eprintln!("  train <training_data.json> - train the neural network");
        eprintln!("  generate <text_description> [output.vdsl] - generate VDSL from text");
        eprintln!("  demo - run demo with sample data");
        return Ok(());
    }
    
    match args[1].as_str() {
        "train" => {
            if args.len() != 3 {
                eprintln!("usage: {} train <training_data.json>", args[0]);
                return Ok(());
            }
            neural_network::train_network(&args[2])?;
        }
        "generate" => {
            let output = if args.len() > 3 { &args[3] } else { "output.vdsl" };
            vdl_generator::generate_from_text(&args[2], output)?;
        }
        "demo" => {
            vdl_generator::run_demo()?;
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
    
    Ok(())
}
