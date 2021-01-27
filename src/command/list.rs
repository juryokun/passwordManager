use std::io;

pub struct ListCommand {}
impl super::Command for ListCommand {
    fn execute(&self) -> Result<String, io::Error> {
        Ok("list, grep, show\n".to_string())
    }
    fn help(&self) -> Result<String, io::Error> {
        let mut relval: String = "".to_string();
        relval = format!("{}{}{}", relval, "show command list", "\n");
        relval = format!("{}{}{}", relval, "example: mpw list", "\n");
        Ok(relval)
    }
}
