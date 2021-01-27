use super::manage_file;
use std::io;

pub trait Command {
    fn execute(&self) -> Result<String, io::Error>;
    fn help(&self) -> Result<String, io::Error>;
}

pub mod grep;
pub mod help;
pub mod list;
// pub mod manage_file;
pub mod show;
