use std::{fs::{read_to_string, File}, io::{LineWriter, Write}};

use d2_converters::runes::converter::convert_line;

mod d2_converters;

fn main() {
    let file_path = "C:\\tmp\\runes d2r.txt";
    
    let read_result = read_to_string(file_path)
        .expect("Couldn't read the d2r runes file. Womp womp");

    let converted: Vec<_> = read_result
        .lines()
        .map(|line| convert_line(line))
        .collect();

    let file = File::create("C:\\tmp\\converted_runes.txt")
        .expect("Couldn't create the converted file to write to");

    let mut file = LineWriter::new(file);


    for line in converted {
        file.write(line.as_bytes()).expect("Couldn't write to file");
        file.write(b"\n").expect("Couldn't write the trailing newline");
    }
    
    file.flush().expect("failed to write the final file to disk");

}
