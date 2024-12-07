use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const max_diff: i32 = 3;
const min_diff: i32 = 1;

fn main() {
    let file_name = "./input.txt";
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Try to read lines of file and push back to cols if successful parse
    if let Ok(lines) = read_lines(file_name){
        for line in lines.flatten() {
            // Split line on spaces and map a parse to i32 on each element, then collect
            reports.push(line.split_whitespace().map(|x| x.parse().unwrap()).collect());
        }
    }

    
    let mut n_safe: i32 = 0;
    for report in reports{

        // Check first diff to see if mutually increasing or decreasing should be criterion
        let first_diff: i32 = report[0] - report[1];
        if first_diff == 0 || first_diff.abs() > max_diff || first_diff.abs() < min_diff {
            continue;
        }
        let change_check: fn(i32, i32) -> bool = if first_diff < 0 {
            less_than
        } else {
            greater_than
        };

        let mut safe: bool = true;
        for i in 1..report.len()-1 {
            if !check_valid(report[i], report[i+1], change_check){
                safe = false;
                continue;
            }
        }
        if safe {
            n_safe += 1;
        }
    }

    println!("n safe: {}", n_safe);
}

fn check_valid(a: i32, b: i32, check: fn(i32,i32) -> bool) -> bool {
    let diff: i32 = a - b;
    if diff == 0 || diff.abs() > max_diff || diff.abs() < min_diff {
        return false
    }
    return check(a, b);
}

fn greater_than(a: i32, b: i32) -> bool{
    return a > b;
}

fn less_than(a: i32, b: i32) -> bool {
    return a < b;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
