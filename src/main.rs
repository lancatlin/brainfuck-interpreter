use std::io::{Read, Write, Seek, SeekFrom, BufWriter, Cursor};
use std::fs::File;
use std::io;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no filename");
        process::exit(1);
    }

    let f = File::open(&args[1])
        .expect("cannot open file");

    let mut interpreter = Interpreter::new(f, io::stdin(), io::stdout());
    interpreter.execute().unwrap();
}

#[derive(Debug)]
pub enum Error {
    MemoryError,
    WriteError,
    ReadError,
    UnknownOperator(String),
}

pub struct Interpreter<T, U, V> where 
    T: Read + Seek,
    U: Read,
    V: Write,
{
    program: T,
    stdin: U,
    stdout: V,
    memory: Vec<u8>,
    index: usize,
    skip: bool,
    stack: Vec<usize>,
    current: usize,
}

impl<T: Read + Seek, U: Read, V: Write> Interpreter<T, U, V> {
    fn new(program: T, stdin: U, stdout: V) -> Interpreter <T, U, V> {
        return Interpreter {
            program,
            stdin,
            stdout,
            memory: vec![0],
            index: 0,
            skip: false,
            stack: Vec::new(),
            current: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.read() {
                Some(operator) => self.operate(operator)?,
                None => { return Ok(()); },
            }
        }
    }

    fn operate(&mut self, operator: u8) -> Result<(), Error> {
        if self.skip && operator != b']' {
            self.skip = false;
            return Ok(())
        }
        match operator {
            b'>' => {
                self.index += 1;
                if self.index == self.memory.len() {
                    self.memory.push(0);
                }
            },
            b'<' => {
                if self.index == 0 {
                    return Err(Error::MemoryError);
                }
                self.index -= 1;
            },
            b'+' => self.add(true),
            b'-' => self.add(false),
            b'.' => {
                if self.stdout.write(&[self.get()]).is_err() {
                    return Err(Error::WriteError)
                }
                return Ok(()) 
            },
            b',' => {
                if let Some(byte) = self.read_byte() {
                    self.set(byte);
                } else {
                    return Err(Error::ReadError);
                }
            },
            b'[' => {
                if self.get() == 0 {
                    self.skip = true;
                } else {
                    self.stack.push(self.current);
                }
            },
            b']' => {
                if self.get() != 0 {
                    self.goback();
                } else {
                    self.stack.pop();
                }
            }
            // _ => { return Err(Error::UnknownOperator(String::from_utf8(vec![operator]).unwrap())) },
            _ => (),
        }
        Ok(())
    }

    fn read_byte(&mut self) -> Option<u8> {
        let mut v = [0; 1];
        match self.stdin.read(&mut v) {
            Ok(_) => Some(v[0]),
            Err(_) => None,
        }
    }

    fn get(&self) -> u8 {
        return self.memory[self.index]
    }

    fn set(&mut self, value: u8) {
        self.memory[self.index] = value
    }

    fn add(&mut self, positive: bool) {
        if positive { self.memory[self.index] += 1 } else { self.memory[self.index] -= 1 }
    }

    fn read(&mut self) -> Option<u8> {
        let mut v = [0; 1];
        if let Ok(n) = self.program.read(&mut v) {
            if n == 1 {
                self.current += 1;
                return Some(v[0]); 
            }
        }
        None
    }

    fn goback(&mut self) {
        let index = self.stack[self.stack.len()-1];
        self.program.seek(SeekFrom::Start(index as u64)).unwrap();
    }
}

fn execute_from_string(program: String, input: String) -> Result<String, Error> {
    let program = Cursor::new(program.as_bytes());
    let input = Cursor::new(input.as_bytes());
    let output = BufWriter::new(Vec::new());
    let mut interpreter = Interpreter::new(program, input, output);
    interpreter.execute()?;
    Ok(String::from_utf8(interpreter.stdout.buffer().to_vec()).unwrap())
}

#[test]
fn test_io() {
    let code = String::from(",.");
    let input = String::from("a");
    assert_eq!("a", execute_from_string(code, input).unwrap());
}

#[test]
fn test_plus() {
    let code = String::from(",++.");
    let input = String::from("a");
    assert_eq!("c", execute_from_string(code, input).unwrap());
}
#[test]
fn test_helloworld() {
    let code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
>++.>+.+++++++..+++.>++.<<+++++++++++++++.
>.+++.------.--------.>+.>.".to_string();
    assert_eq!("Hello World!\n", execute_from_string(code, "".to_string()).unwrap());
}

#[test]
fn test_upper_case() {
    let code = ",----------[----------------------.,----------]".to_string();
    let input = "wancat\n".to_string();
    assert_eq!("WANCAT", execute_from_string(code, input).unwrap());
}

