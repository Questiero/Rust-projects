use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Problem parsing arguments: missing number of discs");
        process::exit(1);
    }

    match args[1].parse::<u8>() {
        Ok(n) => hanoi::run(n),
        Err(_) => {
            eprintln!("Problem parsing arguments: argument cannot be parsed to u8");
            process::exit(1);
        }
    }
}
