use std::io::{Read, Write};

#[derive(Debug)]
pub enum Error {}

pub struct Interpreter<U, V>
where
    U: Read,
    V: Write,
{
    program: Vec<u8>,
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

impl<U: Read, V: Write> Interpreter<U, V> {
    pub fn new<T: Read>(mut program: T, stdin: U, stdout: V) -> Interpreter<U, V> {
        let mut v = Vec::new();
        program.read_to_end(&mut v).expect("Read program fatal");
        Interpreter {
            program: v,
            stdin,
            stdout,
            memory: vec![0],
            index: 0,
            stack: Vec::new(),
            current: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.next() {
                Some(operator) => self.operate(operator)?,
                None => {
                    return Ok(());
                }
            }
        }
    }

    fn next(&mut self) -> Option<u8> {
        match self.program.get(self.current) {
            Some(value) => {
                self.current += 1;
                Some(*value)
            },
            None => None,
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
            b'.' => self.write_to_output(),
            b',' => self.read_from_input(),
            b'[' => self.push_stack(),
            b']' => self.pop_or_goback(),
            _ => (),
        }
        Ok(())
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

    fn write_to_output(&mut self) {
        self.stdout.write(&[self.get()]).expect("write error");
    }

    fn read_from_input(&mut self) {
        let mut v = [0; 1];
        self.stdin.read(&mut v).expect("read error");
        self.set(v[0]);
    }

    fn push_stack(&mut self) {
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

    fn set(&mut self, value: u8) {
        self.memory[self.index] = value
    }

    fn get(&self) -> u8 {
        self.memory[self.index]
    }

    fn goback(&mut self) {
        match self.status() {
            Stack::Index(index) => {
                self.current = *index;
            }
            Stack::Skip => panic!("Should not goback here"),
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
