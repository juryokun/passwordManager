use super::manage_file::get_data_file;
use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use futures::{stream::Stream, Future};
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use serde::Deserialize;
use std::fs;
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
        let key = get_aws_accesskey(&file.home_path);
        upload_file(key.unwrap(), &file.dist_file_path);
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

fn upload_file(key: AwsKey, file_path: &str) -> Result<()> {
    std::env::set_var("AWS_ACCESS_KEY_ID", key.aws_access_key_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", key.aws_secret_access_key);
    let client = S3Client::new(Region::ApNortheast1);
    let mut request = PutObjectRequest::default();
    request.bucket = String::from(key.bucket);
    cfg_if::cfg_if! {
        if #[cfg(test)] {
            request.key = String::from("serviceListTest.enc");
        } else {
            request.key = String::from("serviceList.enc");
        }
    }
    let file: Vec<u8> = fs::read(file_path).unwrap();
    request.body = Some(file.into());

    let _result = client.put_object(request).sync().unwrap();
    Ok(())
}
fn get_aws_accesskey(path: &str) -> Result<AwsKey> {
    let file = format!("{}/{}", path, ".aws.key");
    let key_file = File::open(file)?;
    let mut rdr = csv::Reader::from_reader(key_file);

    let mut key = rdr.deserialize();
    let aws_key: AwsKey = key.next().unwrap().unwrap();
    Ok(aws_key)
}
#[derive(Debug, Deserialize, Clone)]
struct AwsKey {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    bucket: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_upfile() {
        let key = get_aws_accesskey("rsc");
        let rel = upload_file(key.unwrap(), "rsc/serviceList.enc");
        assert_eq!(rel.unwrap(), ());
    }
}
