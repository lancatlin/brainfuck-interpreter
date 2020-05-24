use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub enum Error {}

pub struct Interpreter<T, U, V>
where
    T: Read + Seek,
    U: Read,
    V: Write,
{
    program: T,
    stdin: U,
    stdout: V,
    memory: Vec<u8>,
    index: usize,
    stack: Vec<Stack>,
    current: usize,
}

enum Stack {
    Skip,
    Index(usize),
}

impl<T: Read + Seek, U: Read, V: Write> Interpreter<T, U, V> {
    pub fn new(program: T, stdin: U, stdout: V) -> Interpreter<T, U, V> {
        return Interpreter {
            program,
            stdin,
            stdout,
            memory: vec![0],
            index: 0,
            stack: Vec::new(),
            current: 0,
        };
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.read() {
                Some(operator) => self.operate(operator)?,
                None => {
                    return Ok(());
                }
            }
        }
    }

    fn forward(&mut self) {
        self.index += 1;
        if self.index == self.memory.len() {
            self.memory.push(0);
        }
    }

    fn backward(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        }
    }

    fn write(&mut self) {
        self.stdout.write(&[self.get()]).expect("write error");
    }

    fn push(&mut self) {
        self.stack.push(if self.get() == 0 {
            Stack::Skip
        } else {
            Stack::Index(self.current)
        });
    }

    fn pop_or_goback(&mut self) {
        if self.skip() || self.get() == 0 {
            self.stack.pop();
        } else {
            self.goback();
        }
    }

    fn operate(&mut self, operator: u8) -> Result<(), Error> {
        if self.skip() && operator != b']' && operator != b'[' {
            return Ok(());
        }

        match operator {
            b'>' => self.forward(),
            b'<' => self.backward(),
            b'+' => self.memory[self.index] += 1,
            b'-' => self.memory[self.index] -= 1,
            b'.' => self.write(),
            b',' => self.read_byte(),
            b'[' => self.push(),
            b']' => self.pop_or_goback(),
            _ => (),
        }
        Ok(())
    }

    fn read_byte(&mut self) {
        let mut v = [0; 1];
        self.stdin.read(&mut v).expect("read error");
        self.set(v[0]);
    }

    fn set(&mut self, value: u8) {
        self.memory[self.index] = value
    }

    fn get(&self) -> u8 {
        return self.memory[self.index];
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
        match self.status() {
            Stack::Index(index) => {
                self.current = *index;
                self.program
                    .seek(SeekFrom::Start(self.current as u64))
                    .unwrap();
            }
            _ => (),
        }
    }

    fn status(&self) -> &Stack {
        &self.stack[self.stack.len() - 1]
    }

    fn skip(&self) -> bool {
        if self.stack.is_empty() {
            return false;
        }
        match self.status() {
            Stack::Skip => true,
            Stack::Index(_) => false,
        }
    }
}
