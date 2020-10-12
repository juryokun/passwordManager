use std::error::Error;
use std::io;
use std::process;

fn read() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
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
    if let Err(err) = read() {
        println!("error running read: {}", err);
        process::exit(1);
    }
}
