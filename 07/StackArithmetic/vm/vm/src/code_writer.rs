use super::parser;

pub struct CodeWriter<'a> {
    filename:&'a String,
    command:Vec<String>,
}

impl<'a> CodeWriter<'a> {
    pub fn new(filename: &'a String) -> CodeWriter {
        CodeWriter{
            filename: filename,
            command: Vec::new()
        }
    }

    pub fn write_arithmetc(&mut self, command: String) {
        if command.find("add") != None {
            self.command.push("@SP".to_string());
            self.command.push("A=M".to_string());
            self.command.push("D=M".to_string());
            self.command.push("@SP".to_string());
            self.command.push("M=M-1".to_string());
            self.command.push("A=M".to_string());
            self.command.push("M=M+D".to_string());
        }
    }

    pub fn write_push_pop(&mut self, command_type: parser::CommandType, segment: String, index: isize) {
        match command_type {
            parser::CommandType::CPush => {
                if segment.find("constant") != None {
                    self.command.push(format!("{}{}","@", index.to_string()).to_string());
                    self.command.push("D=A".to_string());
                    self.command.push("@SP".to_string());
                    self.command.push("A=M".to_string());
                    self.command.push("M=D".to_string());
                    self.command.push("@SP".to_string());
                    self.command.push("M=M+1".to_string());
                }
            }
            _ => {}
        }

    }
}

pub enum ArithmeticType {
    ADD,
    SUB,
    ENG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}
