use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

fn load_data() -> Vec<Record> {
    let file = File::open("serviceList.csv");
    let mut rdr = csv::Reader::from_reader(file.unwrap());

    let mut rel: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        rel.push(record);
    }
    rel
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // コマンドが正しいかチェック
    check_command(&args[1]);

    // コマンドの使い方が正しいかチェック
    check_syntax(&args);

    // コマンド実行
    execute(args);

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

fn check_command(command: &String) -> Result<(), Box<Error>> {
    Ok(())
}

fn check_syntax(args: &Vec<String>) -> Result<(), Box<Error>> {
    Ok(())
}

fn execute(args: Vec<String>) -> Result<(), Box<Error>> {
    match &args[1] {
        "grep".to_string() => grep(args)?,
    }
    Ok(())
}

trait Command {
    fn execute(self) -> ();
}
enum Operation {
    List,
    Show,
    Grep,
    Add,
    Delete,
    Update,
}
struct GrepCommand {
    operation: Operation,
    target: String
}

impl Command for GrepCommand {
    fn execute(self) {
        println!("{}", self.target);
    }
}
fn parse (args: &Vec<String>) -> impl Command {
    let arg1: &str = &args[1];
     match arg1 {
        "grep" => GrepCommand {
            operation: Operation::Grep,
            target: args[2].clone(),
        },
        // "show" => Operation::Show,
        // "list" => Operation::List,
        // "add" => Operation::Add,
        // "delete" => Operation::Delete,
        // "update" => Operation::Update,
    }
}

struct GrepOptions {
    target: String,
}

struct ShowOptions {
    service: String,
}

fn grep(args: Vec<String>) -> Result<(), Box<Error>> {
    let re = Regex::new(&args[2]).unwrap();
    let data = load_data();
    for rec in data.iter() {
        let rel = re.find(&rec.service);
        if rel != None {
            println!("{:?}", rec);
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    service: String,
    id: String,
    mail: String,
    password: String,
    memo: String,
}
