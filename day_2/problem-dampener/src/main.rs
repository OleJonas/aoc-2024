use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const max_diff: i32 = 3;
const min_diff: i32 = 1;

/* input:
 * 91 92 90 95 96 safe
 * 8 8 8 8 8 8 8 8 not
 * 8 8 74 75 76 77 not
 * 8 8 6 5 4 3 2 safe
 * 6 7 8 8 9 10 safe
 * 2 3 4 5 safe
 * 9 8 5 5 5 not
 * 1 3 6 8 9 13 13 not
 * 91 92 90 96 97 not
 * 0 safe
 *
 * should be 5 safe
*/

/*
 * edge cases for input now
 * 7 10 8 10 11 safe
 * 29 28 27 25 26 25 22 20 safe
 * 48 46 47 49 51 54 56 safe
 * 1 1 2 3 4 5 safe
 * 1 2 3 4 5 5 safe
 * 5 1 2 3 4 5 safe
 * 1 4 3 2 1 safe
 * 1 6 7 8 9 safe
 * 1 2 3 4 3 safe
 * 9 8 7 6 7 safe
 * 7 10 8 10 11 safe
 * 29 28 27 25 26 25 22 20 safe
 *
 * should be 12 safe
 */


fn main() {
    let file_name = "../input.txt";
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
        
        let error_indices: Vec<i32> = get_errors(&report);
        
        if error_indices.len() > 0 {
            let mut safe: bool = false;
            let mut i: usize = 0;

            while i < report.len() && !safe{
                let mut rep_copy: Vec<i32> = report.to_owned();
                rep_copy.remove(i);
                if get_errors(&rep_copy).len() == 0 {
                    safe = true;
                }
                i += 1;
            }
            if safe {
                println!("safe");
                n_safe += 1;
            } else {
                println!("not safe");
            }
        } else {
            println!("safe");
            n_safe += 1;
        }
        print!("\n");
        
    }
        
        
    println!("n safe: {}", n_safe);
}

fn get_errors(report: &Vec<i32>) -> Vec<i32> {
    // Checks vector of numbers and returns indices of offending elements

    let mut error_indices: Vec<i32> = Vec::new();
    let mut direction: i32 = 0;

    for i in 0..report.len() {
        print!("{} ", report[i]);
    }

    for i in 0..report.len()-1 {
        let diff: i32 = report[i + 1] - report[i];
        let normalized_diff: i32 = if diff != 0 {
            diff / diff.abs()
        } else {
            0
        };

        if direction != 0 && direction != normalized_diff {
            //println!("error at idx {}", i);
            error_indices.push(i as i32);;
            continue;
        } else if direction == 0 {
            direction = normalized_diff;
        }

        if !is_within_bounds(diff) {
            //println!("error at idx {}", i);
            error_indices.push(i as i32);
        }
    }

    return error_indices;
}

fn is_within_bounds(diff: i32) -> bool {
    if diff == 0 || diff.abs() > max_diff || diff.abs() < min_diff {
        return false;
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
