use super::manage_file::*;
use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
extern crate aesstream;
extern crate crypto;

const FILE_SIZE: usize = 1024;
const PASSWORD_SIZE: usize = 32;

pub struct DownloadCommand {
    password: String,
}
impl DownloadCommand {
    pub fn new(args: &Vec<String>) -> Self {
        Self {
            password: args[2].clone(),
        }
    }
}

impl super::Command for DownloadCommand {
    fn execute(&self) -> Result<String> {
        let file = get_data_file();
        decrypt(&file.dist_file_path, &file.file_path, &self.password)?;
        rewite(&file.file_path, &file.home_path)?;
        Ok("Success!".to_string())
    }
    fn help(&self) -> Result<String> {
        Ok("args: password".to_string())
    }
}

// TODO: とりあえず写経しただけなので、修正が必要
// URL: https://qiita.com/readion/items/0834974fe2854282db9e
fn decrypt(dst: &str, src: &str, pass: &str) -> Result<()> {
    let key = pass.as_bytes(); // 引数をバイト変換
    if key.len() > PASSWORD_SIZE {
        println!("Too long password!");
        return Err(std::io::Error::from(std::io::ErrorKind::Other)); // 異常終了なのでエラーを出す
    }
    let mut key_array = [0u8; PASSWORD_SIZE]; // バイト用配列
    for i in 0..key.len() {
        key_array[i] = key[i]; // スライスから配列へ変換
    }

    let src_file = File::open(dst)?; // ファイルを開く
    let decryptor = AesSafe256Decryptor::new(&key_array);
    let mut reader = AesReader::new(&src_file, decryptor)?; // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE]; // 空のバイト配列を用意
    reader.read(&mut block)?; // バイト配列にファイル情報を読み出し

    let dst_file = File::create(src)?; // 出力先ファイルを指定
    let mut writer = BufWriter::new(&dst_file); // 書き込み用の機能を呼び出し
    writer.write(&block)?; // 書き込みを実行
    Ok(())
}

fn rewite(file_path: &str, home_path: &str) -> Result<()> {
    let tmp_file_name = format!("{}/{}", home_path, "tmp.csv");
    let mut rdr = csv::Reader::from_reader(File::open(file_path).unwrap());
    let mut wtr = csv::Writer::from_path(&tmp_file_name).unwrap();

    for result in rdr.deserialize() {
        if let Ok(v) = result {
            let mut record: Record = v;
            let mut bytes: Vec<u8> = record.memo.as_bytes().to_vec();
            if let Some(first) = bytes.iter().position(|&b| b == 0) {
                bytes.truncate(first);
            }
            record.memo = String::from_utf8(bytes).unwrap();
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;

    fs::remove_file(file_path)?;
    fs::rename(&tmp_file_name, file_path)?;
    Ok(())
}
