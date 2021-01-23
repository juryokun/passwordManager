use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::io::{self, BufWriter, Write};
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
    let output = command.execute();

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    write_output(
        &mut stdout,
        output.unwrap_or("コマンドに失敗しました。".to_string()),
    )
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
    //     print_output("error running read: {}", err);
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
    #[cfg(not(test))]
    fn file_open(&self) -> Result<File, std::io::Error> {
        File::open(&self.file_path)
    }
    #[cfg(test)]
    fn file_open(&self) -> Result<File, std::io::Error> {
        File::open("rsc/serviceList.csv")
    }
}

trait Command {
    fn execute(&self) -> Result<String, io::Error>;
    fn help(&self) -> Result<String, io::Error>;
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

struct ListCommand {}
impl Command for ListCommand {
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

struct HelpCommand {}
impl Command for HelpCommand {
    fn execute(&self) -> Result<String, io::Error> {
        Ok("below...\n".to_string())
    }
    fn help(&self) -> Result<String, io::Error> {
        self.execute()
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

fn write_output<W: Write>(w: &mut W, output: String) {
    write!(w, "{}", output);
}
// fn write_output<W: Write>(w: &mut W, output: String) {
//     write!(w, "{}", output);
// }

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    service: String,
    id: String,
    mail: String,
    password: String,
    memo: String,
}

#[test]
fn test_grep() {
    let args: Vec<String> = vec!["".to_string(), "".to_string(), "le".to_string()];
    let command = GrepCommand::new(&args);
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    assert_eq!(buf, b"google\napple\n");
}

#[test]
fn test_show() {
    let args: Vec<String> = vec!["".to_string(), "".to_string(), "google".to_string()];
    let command = ShowCommand::new(&args);
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    let check_str = "Record { service: \"google\", id: \"google_id\", mail: \"google_mail\", password: \"google_password\", memo: \"google_memo\" }\n";
    assert_eq!(String::from_utf8(buf).unwrap(), check_str);
}

#[test]
fn test_list() {
    let command = ListCommand {};
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    assert_eq!(buf, b"list, grep, show\n");
}
