use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

const FILE_NAME: &str = "serviceList.csv";

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub service: String,
    pub id: String,
    pub mail: String,
    pub password: String,
    pub memo: String,
}

struct DataFile {
    file_path: String,
}
impl DataFile {
    fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                let file_path = "rsc/serviceList.csv".to_string();
            } else {
                let file_path = format!("{}/{}",Self::get_home_path(), FILE_NAME);
            }
        }
        Self {
            file_path: file_path,
        }
    }
    #[cfg(any(unix))]
    fn get_home_path() -> String {
        let home = std::env::var("HOME");
        home.unwrap()
    }
    #[cfg(target_os = "windows")]
    fn get_home_path() -> String {
        let userprofile = std::env::var("USERPROFILE");
        userprofile.unwrap()
    }
    fn file_open(&self) -> Result<File, std::io::Error> {
        File::open(&self.file_path)
    }
}
pub fn load_data() -> Vec<Record> {
    let file = DataFile::new();
    let mut rdr = csv::Reader::from_reader(file.file_open().unwrap());

    let mut rel: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        rel.push(record);
    }
    rel
}
