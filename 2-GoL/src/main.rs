use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Problem parsing arguments: missing file path");
        process::exit(1);
    } else {
        let _ = game_of_life::run(&args[1]);
    }

}
