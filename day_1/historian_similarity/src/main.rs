use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "./input.txt";
    let mut col1: Vec<u32> = Vec::new();
    let mut col2_occurrences: HashMap<u32,u32> = HashMap::new();
    //let mut col2: Vec<u32> = Vec::new();

    // Try to read lines of file and push back to cols if successful parse
    if let Ok(lines) = read_lines(file_name){
        for line in lines.flatten() {

            // Split line on spaces and collect() to make them a &str vec that we can then parse
            let tmp: Vec<&str> = line.split_whitespace().collect();
            col1.push(tmp[0].parse().unwrap());
            
            let num: u32 = tmp[1].parse().unwrap();
            let occurrences = col2_occurrences.entry(num).or_insert(0); //col2.push(tmp[1].parse().unwrap());
            *occurrences += 1;
        }
    }

    let mut importance: u32 = 0;
    for num in col1{
        match col2_occurrences.get(&num) {
            Some(n) => importance += num * n,
            None => ()
        }
    }

    println!("importance: {}", importance);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

