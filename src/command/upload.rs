use super::manage_file::get_data_file;
use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
extern crate aesstream;
extern crate crypto;

const FILE_SIZE: usize = 1024;
const PASSWORD_SIZE: usize = 32;

pub struct UploadCommand {
    password: String,
}
impl UploadCommand {
    pub fn new(args: &Vec<String>) -> Self {
        Self {
            password: args[2].clone(),
        }
    }
}

impl super::Command for UploadCommand {
    fn execute(&self) -> Result<String> {
        let file = get_data_file();
        encrypt(&file.file_path, &file.dist_file_path, &self.password);
        Ok("Success!".to_string())
    }
    fn help(&self) -> Result<String> {
        Ok("args: password".to_string())
    }
}

// TODO: とりあえず写経しただけなので、修正が必要
// URL: https://qiita.com/readion/items/0834974fe2854282db9e
fn encrypt(src: &str, dist: &str, pass: &str) -> Result<()> {
    let src_file = File::open(src)?;
    let mut reader = BufReader::new(&src_file);
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];
    reader.read(&mut block)?;

    let key = pass.as_bytes();
    if key.len() > PASSWORD_SIZE {
        println!("Too long password!");
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }
    let mut key_array = [0u8; PASSWORD_SIZE];
    for i in 0..key.len() {
        key_array[i] = key[i];
    }

    let dst_file = File::create(dist)?;
    let encryptor = AesSafe256Encryptor::new(&key_array);
    let mut writer = AesWriter::new(dst_file, encryptor)?;
    writer.write_all(&block)?;
    Ok(())
}
