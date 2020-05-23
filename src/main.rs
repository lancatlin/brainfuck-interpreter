use std::io::{Read, Write, Seek, BufWriter, Cursor};
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub enum Error {
    MemoryError,
    WriteError,
    ReadError,
    UnknownOperator(String),
}

pub struct Interpreter<T, U, V>
    where T: Read + Seek,
          U: Read,
          V: Write,
{
    program: T,
    stdin: U,
    stdout: V,
    memory: Vec<u8>,
    index: usize,
}

impl<T: Read + Seek, U: Read, V: Write> Interpreter<T, U, V> {
    fn new(program: T, stdin: U, stdout: V) -> Interpreter <T, U, V> {
        return Interpreter {
            program,
            stdin,
            stdout,
            memory: vec![0],
            index: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.next() {
                Some(operator) => self.operate(operator)?,
                None => { return Ok(()); },
            }
        }
    }

    fn operate(&mut self, operator: u8) -> Result<(), Error> {
        println!("{}", operator);
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
            b'+' => self.memory[self.index] += 1,
            b'-' => self.memory[self.index] -= 1,
            b'.' => {
                if self.stdout.write(&[self.memory[self.index]]).is_err() {
                    return Err(Error::WriteError)
                }
                return Ok(()) 
            },
            b',' => {
                if let Some(byte) = self.read_byte() {
                    self.memory[self.index] = byte;
                } else {
                    return Err(Error::ReadError);
                }
            },
            0 => (),
            _ => { return Err(Error::UnknownOperator(format!("{}", operator))) },
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
}

impl<T, U, V> Iterator for Interpreter<T, U, V> where
    T: Read + Seek,
    U: Read,
    V: Write,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = [0; 1];
        match self.program.read(&mut v) {
            Ok(n) => {
                if n != 1 { return None }
                Some(v[0])
            },
            Err(_) => None,
        }
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
fn test_helloworld() {
    let code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
>++.>+.+++++++..+++.>++.<<+++++++++++++++.
>.+++.------.--------.>+.>.".to_string();
    assert_eq!("Hello World!", execute_from_string(code, "".to_string()).unwrap());
}
