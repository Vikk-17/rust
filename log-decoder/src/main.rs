use clap::Parser;
use std::{
    fs,
    collections::HashMap,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file: String,

    #[arg(long)]
    program: String,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file)
        .expect("Failed to read log files");

    let lines: Vec<String> = content.lines()
        .map(|l| l.to_string())
        .collect();
    // println!("{:#?}", lines);
    let program_logs = extract_program_logs(&lines, &args.program);

    if program_logs.is_empty() {
        println!("No logs found for program {}", args.program);
        return;
    }

    let decoded = decode_logs(program_logs);
    println!("\nProgram ID: {}\n", args.program);
    for (key, value) in decoded {
        println!("{}: {}", key, value);
    }
}


/// Extract logs belonging to a specific program
fn extract_program_logs(lines: &[String], program: &str) -> Vec<String> {
    let mut inside_program = false;
    let mut logs = Vec::new();

    for line in lines {
        // program invocation
        if line.contains(program) && line.contains("invoked") {
            inside_program = true;
            continue;
        }

        // program finishes
        if inside_program && line.contains("success") {
            break;
        }

        // capture program logs
        if inside_program && line.contains("Program log:") {
            let cleaned = line
                .replace("Program log:", "")
                .trim()
                .to_string();
            logs.push(cleaned);
        }
    }

    logs
}

/// convert raw logs into structured key-value data
fn decode_logs(logs: Vec<String>) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for log in logs {
        if log.starts_with("Instruction:") {
            let instruction = log
                .replace("Instruction:", "")
                .trim()
                .to_string();
            result.insert("instruction".to_string(), instruction);
            continue;
        }

        if let Some((key, value)) = log.split_once("=") {
            result.insert(key.to_string(), value.to_string());
        }
    }
    result
}
