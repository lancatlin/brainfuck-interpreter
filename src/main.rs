use brainfuck;
use std::env;
use std::fs::File;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no filename");
        process::exit(1);
    }

    match File::open(&args[1]) {
        Ok(f) => {
            let mut interpreter = brainfuck::Interpreter::new(f, io::stdin(), io::stdout());
            interpreter.execute().unwrap();
        },
        Err(e) => {
            println!("Open file error: {}", e);
            process::exit(1);
        },
    }

}
