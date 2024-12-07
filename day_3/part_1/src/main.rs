use regex::Regex;
use std::fs;

fn main() {

    // regex to match mul(A,B) pattern
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let contents: String = fs::read_to_string("../input.txt").expect("Error reading file");

    let mut total_sum: i32 = 0;
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    for m in re.captures_iter(&contents).map(|c| c.get(0).unwrap().as_str()).collect::<Vec<&str>>() {
        let nums: Vec<i32> = num_re.captures_iter(m).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect::<Vec<i32>>();
        let product: i32 = nums[0] * nums[1];
        total_sum += product;
    }
    println!("{}", total_sum);

}
