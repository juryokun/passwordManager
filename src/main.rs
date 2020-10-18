use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

fn read(data: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(data);
    for result in rdr.records() {
        let records = result?;
        let title = &records[0];
        let body = &records[1];
        println!("title: {}", &title);
        println!("body: {}", &body);
    }
    Ok(())
}

fn main() {
    let data_csv = File::open("data.csv");
    if let Err(err) = read(data_csv.unwrap()) {
        println!("error running read: {}", err);
        process::exit(1);
    }
}
