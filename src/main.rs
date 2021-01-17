use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::env;
// use std::error::Error;
use std::fs::File;

const FILE_NAME: &str = "serviceList.csv";

fn load_data() -> Vec<Record> {
    let file = DataFile::new();
    let mut rdr = csv::Reader::from_reader(file.file_open().unwrap());

    let mut rel: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        rel.push(record);
    }
    rel
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 引数から実行するコマンドを判定する
    let command = parse_to_command(&args);
    // コマンド実行
    command.execute();
    // execute(args);

    // let write_csv = File::open("data.csv");
    // let mut wtr = csv::Writer::from_writer(vec![]);
    // let mut wtr = csv::Writer::from_path("write.csv").unwrap();
    // for rec in rel.iter() {
    //     wtr.serialize(rec);
    // }
    // wtr.serialize(Record {
    //     city: "hokkaido".to_string(),
    //     region: "asia".to_string(),
    //     country: "japan".to_string(),
    // });
    // wtr.into_inner();
    // wtr.flush();
    // if let Err(err) = read(data_csv.unwrap()) {
    //     println!("error running read: {}", err);
    //     process::exit(1);
    // }
}

struct DataFile {
    name: String,
    home_path: String,
    file_path: String,
}
impl DataFile {
    fn new() -> Self {
        let name = FILE_NAME.to_string();
        let home_path = Self::get_home_path();
        let file_path = format!("{}/{}", home_path, name);
        Self {
            name: name,
            home_path: home_path,
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

trait Command {
    fn execute(&self) -> ();
    fn help(&self) -> ();
}

struct GrepCommand {
    target: String,
}
impl GrepCommand {
    fn new(args: &Vec<String>) -> Self {
        Self {
            target: args[2].to_lowercase(),
        }
    }
}
impl Command for GrepCommand {
    fn execute(&self) {
        let re = Regex::new(&self.target).unwrap();
        let data = load_data();
        for rec in data.iter() {
            let t = &rec.service.to_lowercase();
            let rel = re.find(t);
            if rel != None {
                println!("{}", rec.service);
            }
        }
    }
    fn help(&self) {
        println!("grep service");
        println!("example: mpw grep [search_string]");
    }
}

struct ShowCommand {
    target: String,
}
impl ShowCommand {
    fn new(args: &Vec<String>) -> Self {
        Self {
            target: args[2].to_lowercase(),
        }
    }
}
impl Command for ShowCommand {
    fn execute(&self) {
        let data = load_data();
        for rec in data.iter() {
            if self.target == rec.service.to_lowercase() {
                println!("{:?}", rec);
                return;
            }
        }
    }
    fn help(&self) {
        println!("show sevice id, pass, mail and memo");
        println!("example: mpw show [service]");
    }
}

struct ListCommand {}
impl Command for ListCommand {
    fn execute(&self) {
        println!("list, grep, show");
    }
    fn help(&self) {
        println!("show command list");
        println!("example: mpw list");
    }
}

struct HelpCommand {}
impl Command for HelpCommand {
    fn execute(&self) {
        println!("below...");
    }
    fn help(&self) {
        self.execute();
    }
}

fn parse_to_command(args: &Vec<String>) -> Box<dyn Command> {
    match &*args[1] {
        "show" => Box::new(ShowCommand::new(args)),
        "grep" => Box::new(GrepCommand::new(args)),
        "list" => Box::new(ListCommand {}),
        // "delete" => Operation::Delete,
        // "update" => Operation::Update,
        _ => Box::new(HelpCommand {}),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    service: String,
    id: String,
    mail: String,
    password: String,
    memo: String,
}

#[test]
fn test_travis() {
    let rel = 1 + 1;
    assert_eq!(rel, 2);
}
