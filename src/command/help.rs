use std::io;

pub struct HelpCommand {}
impl super::Command for HelpCommand {
    fn execute(&self) -> Result<String, io::Error> {
        Ok("below...\n".to_string())
    }
    fn help(&self) -> Result<String, io::Error> {
        self.execute()
    }
}
