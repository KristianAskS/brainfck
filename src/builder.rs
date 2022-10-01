use std::fs::File;
use std::io::Write;

use crate::parser::Instruction;

pub struct Builder {
    instructions: Vec<Instruction>,
}

impl Builder {
    pub fn new(is: Vec<Instruction>) -> Self {
        Self { instructions: is }
    }

    pub fn builder(&self, filename: &str) {
        let mut file: File = File::create(filename).unwrap();
        let mut contents: String = String::new();
        self.add_header(&mut contents);

        for instruction in self.instructions.iter() {
            match instruction.a {
                '>' => self.add_right(&mut contents, instruction.repeat),
                '<' => self.add_left(&mut contents, instruction.repeat),
                '+' => self.add_inc(&mut contents, instruction.repeat),
                '-' => self.add_dec(&mut contents, instruction.repeat),
                '.' => self.add_print(&mut contents),
                ',' => self.add_read(&mut contents),
                '[' => self.add_loop_start(&mut contents),
                ']' => self.add_loop_end(&mut contents),
                _ => {}
            }
        }
        self.add_ending(&mut contents);
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn add_loop_start(&self, contents: &mut String) {
        contents.push_str("while (*ptr) {\n");
    }

    fn add_loop_end(&self, contents: &mut String) {
        contents.push_str("}\n");
    }

    fn add_print(&self, contents: &mut String) {
        contents.push_str("putchar(*ptr);\n");
    }

    fn add_read(&self, contents: &mut String) {
        contents.push_str("*ptr = getchar();\n");
    }

    fn add_right(&self, contents: &mut String, repeat: u8) {
        contents.push_str(&format!("ptr += {};\n", repeat));
    }

    fn add_left(&self, contents: &mut String, repeat: u8) {
        contents.push_str(&format!("ptr -= {};\n", repeat));
    }

    fn add_inc(&self, contents: &mut String, repeat: u8) {
        contents.push_str(&format!("*ptr += {};\n", repeat));
    }
    fn add_dec(&self, contents: &mut String, repeat: u8) {
        contents.push_str(&format!("*ptr -= {};\n", repeat));
    }

    fn add_header(&self, contents: &mut String) {
        contents.push_str("#include <stdio.h>\n\n");
        contents.push_str("int main() {\n");
        contents.push_str("char array[30000] = {0};\n");
        contents.push_str("char *ptr = array;\n");
    }

    fn add_ending(&self, contents: &mut String) {
        contents.push_str("    return 0;\n");
        contents.push_str("}\n");
    }
}
