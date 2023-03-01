use std::{
    collections::HashMap,
    io::{self, Write},
    num::Wrapping,
};

pub struct VirtualMachine {
    memory: Vec<Wrapping<u8>>,
    code: Vec<char>,
    dc: usize, // data ptr
    pc: usize, // code ptr
}

impl VirtualMachine {
    pub fn new(code: &str) -> Self {
        Self {
            memory: vec![Wrapping(0); 1000],
            code: code.chars().collect(),
            dc: 0,
            pc: 0,
        }
    }

    pub fn interpret(&mut self) {
        let mut brackets_cache = HashMap::new();
        fill_brackets_cache(&self.code, &mut brackets_cache);

        let code_len = self.code.len();
        while self.pc < code_len {
            self.execute(&mut brackets_cache);
        }
    }

    fn execute(&mut self, brackets_cache: &mut HashMap<usize, usize>) {
        match self.code[self.pc] {
            '>' => self.dc += 1,
            '<' => self.dc -= 1,
            '+' => self.memory[self.dc] += 1,
            '-' => self.memory[self.dc] -= 1,
            '.' => {
                print!("{}", self.memory[self.dc].0 as char);
                io::stdout().flush().unwrap();
            }
            ',' => {
                let mut tmp_str = String::new();
                io::stdin().read_line(&mut tmp_str).unwrap();
                let c = tmp_str.trim().chars().next().unwrap();
                self.memory[self.dc] = Wrapping(c as u8);
            }
            '[' => {
                if self.memory[self.dc].0 == 0 {
                    self.pc = *brackets_cache.get(&self.pc).unwrap();
                }
            }
            ']' => {
                if self.memory[self.dc].0 != 0 {
                    self.pc = *brackets_cache.get(&self.pc).unwrap();
                }
            }
            _ => (),
        }

        self.pc += 1;
    }
}

fn fill_brackets_cache(code: &[char], brackets_cache: &mut HashMap<usize, usize>) {
    let mut stack = Vec::new();

    code.iter().enumerate().for_each(|(i, c)| match c {
        '[' => stack.push(i),
        ']' => {
            let Some(left) = stack.pop() else {
                panic!("Unmatched brackets at position {i}");
            };
            brackets_cache.insert(left, i);
            brackets_cache.insert(i, left);
        }
        _ => (),
    });
}
