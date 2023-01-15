use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("wc")
        .version("1.0")
        .author("Rajesh Kumar <")
        .about("Word, line, and byte count")
        .arg(
            Arg::with_name("file")
                .required(true)
                .index(1)
                .help("The file to be processed"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Print the byte count"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Print the word count"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Print the line count"),
        )
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let file = File::open(file_name).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        line_count += 1;
        word_count += line.split_whitespace().count();
        byte_count += line.as_bytes().len();
    }

    if matches.is_present("lines") {
        println!("{}", line_count);
    }
    if matches.is_present("words") {
        println!("{}", word_count);
    }
    if matches.is_present("bytes") {
        println!("{}", byte_count);
    }
    if !matches.is_present("lines") && !matches.is_present("words") && !matches.is_present("bytes") {
        println!("{} {} {} {}", line_count, word_count, byte_count, file_name);
    }
}
