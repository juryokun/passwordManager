use std::env;
use std::io::{self, BufWriter, Write};
// use std::error::Error;

mod command;
mod manage_file;
use command::*;

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

fn parse_to_command(args: &Vec<String>) -> Box<dyn Command> {
    match &*args[1] {
        "show" => Box::new(show::ShowCommand::new(args)),
        "grep" => Box::new(grep::GrepCommand::new(args)),
        "list" => Box::new(list::ListCommand {}),
        // "delete" => Operation::Delete,
        // "update" => Operation::Update,
        _ => Box::new(help::HelpCommand {}),
    }
}

fn write_output<W: Write>(w: &mut W, output: String) {
    write!(w, "{}", output);
}
// fn write_output<W: Write>(w: &mut W, output: String) {
//     write!(w, "{}", output);
// }

#[test]
fn test_grep() {
    let args: Vec<String> = vec!["".to_string(), "".to_string(), "le".to_string()];
    let command = grep::GrepCommand::new(&args);
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    assert_eq!(buf, b"google\napple\n");
}

#[test]
fn test_show() {
    let args: Vec<String> = vec!["".to_string(), "".to_string(), "google".to_string()];
    let command = show::ShowCommand::new(&args);
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    let check_str = "Record { service: \"google\", id: \"google_id\", mail: \"google_mail\", password: \"google_password\", memo: \"google_memo\" }\n";
    assert_eq!(String::from_utf8(buf).unwrap(), check_str);
}

#[test]
fn test_list() {
    let command = list::ListCommand {};
    let output = command.execute();
    let mut buf = Vec::new();
    write_output(&mut buf, output.unwrap());

    assert_eq!(buf, b"list, grep, show\n");
}
