

pub struct Parser<'a> {
    current_command:String,
    command:Vec<&'a str>,
    index:usize,
}

impl<'a> Parser<'a> {
    pub fn new(command:Vec<&'a str>) -> Parser {
        Parser {
            current_command:"".to_string(),
            command,
            index:0,
        }
    }

    pub fn has_more_command(&mut self) -> bool {
        let i = self.index;
        let len = self.command.len() - 1;
        if len == i {
            return false;
        }
        return true;
    }

    pub fn advance(&mut self) {
        self.current_command = self.command[self.index].replace(" ", "");
        self.index = self.index + 1;
    }

    pub fn command_type(&self) -> CommandType {
        if self.current_command.find("push") != None {
            return CommandType::CPush;
        } else if self.current_command.find("pop")  != None {
            return CommandType::CPop;
        } else if self.current_command.find("label") != None {
            return CommandType::CLabel;
        } else {
            return CommandType::CArithmetic;
        }
    }

    pub fn arg1(&self) -> String {
        let command = &self.current_command;
        if self.current_command.find(' ') != None {
            let v: Vec<&str> = command.split(' ').collect();
            return v[1].to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn arg2(&self) -> String {
        let command = &self.current_command;
        if self.current_command.find(' ') != None {
            let v: Vec<&str> = command.split(' ').collect();
            return v[2].to_string();
        } else {
            return "".to_string();
        }
    }
}

pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
}
