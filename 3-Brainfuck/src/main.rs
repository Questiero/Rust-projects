use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Problem parsing arguments: missing file path");
        process::exit(1);
    } else {
        let contents =
            fs::read_to_string(&args[1]).unwrap_or_else(|_| panic!("Cannot read file {}", args[1]));
        brainfuck::interpret(&contents);
    }
}
