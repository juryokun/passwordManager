use super::manage_file::load_data;
use regex::Regex;
use std::io;

pub struct GrepCommand {
    target: String,
}
impl GrepCommand {
    pub fn new(args: &Vec<String>) -> Self {
        Self {
            target: args[2].to_lowercase(),
        }
    }
}
impl super::Command for GrepCommand {
    fn execute(&self) -> Result<String, io::Error> {
        let mut relval: String = "".to_string();
        let re = Regex::new(&self.target).unwrap();
        let data = load_data();
        for rec in data.iter() {
            let t = &rec.service.to_lowercase();
            let rel = re.find(t);
            if rel != None {
                relval = format!("{}{}{}", relval, rec.service, "\n");
            }
        }
        Ok(relval)
    }
    fn help(&self) -> Result<String, io::Error> {
        let mut relval = "".to_string();
        relval = format!("{}{}{}", relval, "grep service", "\n");
        relval = format!("{}{}{}", relval, "example: mpw grep [search_string]", "\n");
        Ok(relval)
    }
}
