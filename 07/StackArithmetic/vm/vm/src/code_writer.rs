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

    pub fn write_arithmetc(command: String) {

    }

    pub fn write_push_pop(command_type: parser::CommandType, segment: String, index: isize) {
    }
}
