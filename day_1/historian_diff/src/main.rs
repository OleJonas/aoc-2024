//use std::io;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "./input.txt";
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    // Try to read lines of file and push back to cols if successful parse
    if let Ok(lines) = read_lines(file_name){
        for line in lines.flatten() {

            // Split line on spaces and collect() to make them a &str vec that we can then parse
            let tmp: Vec<&str> = line.split_whitespace().collect();

            col1.push(tmp[0].parse().unwrap());
            col2.push(tmp[1].parse().unwrap());
        }
    }

    col1.sort();
    col2.sort();
    for i in 0..col1.len(){
        println!("{} {}", col1[i], col2[i]);
    }

    let mut diff: u32 = 0;
    for i in 0..col1.len(){
        diff += col1[i].abs_diff(col2[i]);
    }

    println!("diff: {}", diff);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
