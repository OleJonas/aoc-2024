use regex::Regex;
use std::fs;



const TEST_STRING: &str = "blablaxmul(1,2)dont()mul(1,3)do()mul(1,4)"; // End sum should be 6
//const TEST_STRING2: &str = "lol";

fn main() {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    let mut i: usize = 0;
    let mut do_idx: usize = 0;
    let mut dont_idx: usize = 0;
    let mut mul_idx: usize = 0;

    let mut dont: bool = false;
    let mut sum: i32 = 0;

    let contents: String = fs::read_to_string("../input.txt").expect("Error reading file");

    while i < contents.len() {
        // dont regexing
        match dont_re.find_at(&contents, i) {
            Some(m) => {
                dont_idx = m.end(); // Don't need to unwrap here as it is handled in the match clause
            }
            None => {
                dont_idx = usize::MAX;
            }
        }

        // do regexing
        match do_re.find_at(&contents, i) {
            Some(m) => {
                do_idx = m.end(); // Don't need to unwrap here as it is handled in the match clause
            }
            None => {
                do_idx = usize::MAX;
            }
        }    


        // mul regexing
        match mul_re.find_at(&contents, i) {
            Some(m) => {
                mul_idx = m.end(); // Don't need to unwrap here as it is handled in the match clause
            }
            None => {
                mul_idx = usize::MAX;
            }
        }

        if (mul_idx < dont_idx) || (dont_idx < do_idx && do_idx < mul_idx) {
            let mul_stmt: &str = mul_re.captures_at(&contents, i).unwrap().get(0).unwrap().as_str();
            let factors: Vec<i32> = num_re.captures_iter(mul_stmt).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect::<Vec<i32>>();
            sum += (factors[0] * factors[1]);
            i = mul_idx;
            println!("Found mul: {}, product is: {}", mul_stmt, factors[0] * factors[1]);
        } else if do_idx < usize::MAX && do_idx >= dont_idx {
            i = do_idx;
            println!("No mul found outside of dont statement, set to do");
        } else if do_idx == usize::MAX && dont_idx < do_idx {
            println!("done");
            break;
        } else if do_idx < dont_idx {
            i = do_idx;
        } else if mul_idx == usize::MAX {
            println!("done");
            break;
        }
    }
    
    println!("Done with search, sum is: {}", sum);
}

