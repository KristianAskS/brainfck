use std::cell::{RefCell, RefMut};

#[derive(Copy, Clone)]
pub struct Instruction {
    pub a: char,
    pub repeat: u8,
}

#[derive(Clone)]
pub struct Parser {
    string: RefCell<String>,
    instructions: RefCell<Vec<Instruction>>,
}

impl Parser {
    pub fn new(string: String) -> Parser {
        Parser {
            string: RefCell::new(string),
            instructions: RefCell::new(Vec::new()),
        }
    }

    fn is_valid_instruction(&self, c: &char) -> bool {
        let valid_instructions: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];
        valid_instructions.contains(&c)
    }

    pub fn get_instructions(&self) -> Vec<Instruction> {
        self.instructions.borrow().to_vec()
    }
    fn get_char(&self, nth: usize) -> char {
        self.string.borrow().chars().nth(nth).unwrap()
    }

    pub fn parse(&self) {
        let mut loop_stack: Vec<usize> = Vec::new();
        let mut instructions: RefMut<Vec<Instruction>> = self.instructions.borrow_mut();

        let mut i: usize = 0;
        while i < self.string.borrow().len() {
            let c = self.get_char(i);
            if c == ' ' {
                instructions.push(Instruction { a: c, repeat: 0 });
                continue;
            }

            if !self.is_valid_instruction(&c) {
                panic!("invalid instruction: {}", c);
            }

            if c == '[' {
                loop_stack.push(instructions.len());
                instructions.push(Instruction { a: c, repeat: 0 });
                i += 1;
            } else if c == ']' {
                loop_stack.pop().unwrap();
                instructions.push(Instruction { a: c, repeat: 0 });
                i += 1;
            } else {
                let mut repeat: u8 = 1;
                if c == '+' || c == '-' || c == '>' || c == '<' {
                    i += 1;
                    while i < self.string.borrow().len() && self.get_char(i) == c {
                        repeat += 1;
                        i += 1;
                    }
                    i -= 1;
                }
                instructions.push(Instruction {
                    a: c,
                    repeat: repeat,
                });
                i += 1;
            }
        }
        if loop_stack.len() != 0 {
            panic!("Unmatched loop");
        }
    }
}
