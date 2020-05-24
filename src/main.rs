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

    let f = File::open(&args[1]).expect("cannot open file");

    let mut interpreter = brainfuck::Interpreter::new(f, io::stdin(), io::stdout());
    interpreter.execute().unwrap();
}
