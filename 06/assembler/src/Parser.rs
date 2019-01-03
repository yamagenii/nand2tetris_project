

pub struct Parser<'a> {
    currentCommand:String,
    command:Vec<&'a str>,
    index:usize,
}

impl<'a> Parser<'a> {
    pub fn new(command:Vec<&'a str>) -> Parser {
        Parser{
            currentCommand:"".to_string(),
            command,
            index:0
        }

    }
    pub fn hasMoreCommands(&mut self) -> bool {
        let i = self.index;
        let len = self.command.len() - 1;
        if len == i {
            return false;
        }
        return true;
    }

    pub fn advance(&mut self) {
        self.currentCommand = self.command[self.index].replace(" ","");
        self.index = self.index + 1;

    }

    pub fn commandType(&self) -> CommandType {
        if self.currentCommand.find('@') != None {
            return CommandType::A_COMMAND;
        } else if self.currentCommand.find('=') != None || self.currentCommand.find(';') != None {
            return CommandType::C_COMMAND;
        } else {
            return CommandType::L_COMMAND;
        }
    }

    pub fn symbol(&self) -> String {
        let command = &self.currentCommand;
        return command.replace(|c:char| !c.is_alphanumeric(), "");
    }
    
    pub fn dest(&self) -> String {
        let command = &self.currentCommand;
        if self.currentCommand.find('=') != None {
            let v: Vec<&str> = command.split('=').collect();
            return v[0].to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn jump(&self) -> String {
        let command = &self.currentCommand;
        if self.currentCommand.find(';') != None {
            let v: Vec<&str> = command.split(';').collect();
            return v[1].to_string();
        } else {
            return "".to_string();
        }
    }
    pub fn comp(&self) -> String {
        let command = &self.currentCommand;
        if self.currentCommand.find(';') != None && self.currentCommand.find('=') != None {
            let v: Vec<&str> = command.split(';').collect();
            let v: Vec<&str> = v[0].split('=').collect();
            return v[1].to_string();
        } else if self.currentCommand.find('=') != None {
            let v: Vec<&str> = command.split('=').collect();
            return v[1].to_string();
        } else {
            let v: Vec<&str> = command.split(';').collect();
            return v[0].to_string();
        }
    }
}

pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}
