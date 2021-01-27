use super::manage_file::load_data;
use std::io;

pub struct ShowCommand {
    target: String,
}
impl ShowCommand {
    pub fn new(args: &Vec<String>) -> Self {
        Self {
            target: args[2].to_lowercase(),
        }
    }
}
impl super::Command for ShowCommand {
    fn execute(&self) -> Result<String, io::Error> {
        let data = load_data();
        for rec in data.iter() {
            if self.target == rec.service.to_lowercase() {
                return Ok(format!("{:?}{}", rec, "\n"));
            }
        }
        Ok("".to_string())
    }
    fn help(&self) -> Result<String, io::Error> {
        let mut relval: String = "".to_string();
        relval = format!(
            "{}{}{}",
            relval, "show sevice id, pass, mail and memo", "\n"
        );
        relval = format!("{}{}{}", relval, "example: mpw show [service]", "\n");
        Ok(relval)
    }
}
