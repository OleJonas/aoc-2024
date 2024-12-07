use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const xmas: [char; 4] = ['X','M','A','S'];
const directions: [[i32; 2]; 8] = [
    [0,1],   // e
    [1,1],   // se   
    [1,0],   // s
    [1,-1],  // sw
    [0,-1],  // w
    [-1,-1], // nw
    [-1,0],  // n
    [-1,1]   // ne
];

fn main() {
    const file_name: &str = "../input.txt";
    let mut xmas_matrix: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(file_name){
        for line in lines.flatten() {
            xmas_matrix.push(line.chars().collect());
        }
    }

    let mut count: u32 = 0;
    for i in 0..xmas_matrix.len() {
        for j in 0..xmas_matrix[i].len() {
            if xmas_matrix[i][j] == xmas[0] {
                count += check_for_xmas(&xmas_matrix, [i,j]);
            }
        }
    }
    println!("Amount of xmas: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn check_for_xmas(xmas_matrix: &Vec<Vec<char>>, pos: [usize; 2]) -> u32 {
    let mut counter: u32 = 0;

    for direction in directions {
        let mut tmp_pos: [usize; 2] = pos.clone();
        println!("POS: [{},{}], DIR: [{},{}]", tmp_pos[0], tmp_pos[1], direction[0], direction[1]);
        let mut is_xmas: bool = true;
        for step in 1..xmas.len(){ // start at m :)
            //print!("{} ", xmas[step]);
            match get_next(xmas_matrix, tmp_pos, direction, step) {
                Some(new_pos) => {
                    tmp_pos = new_pos;
                }
                None => {
                    is_xmas = false;
                    break;
                }
            }
        }
        if is_xmas {
            counter += 1;
            println!("Found for pos: [{},{}], dir: [{},{}]", tmp_pos[0], tmp_pos[1], direction[0], direction[1]);
        }
    }
    return counter;
}


fn get_next(xmas_matrix: &Vec<Vec<char>>, pos: [usize; 2], direction: [i32; 2], step: usize) -> Option<[usize; 2]> {
    // Check for out of bounds first

    let tmp_y: i32 = pos[0] as i32 + direction[0];
    let tmp_x: i32 = pos[1] as i32 + direction[1];

    let new_y: usize;
    let new_x: usize;

    if usize::try_from(tmp_y).is_ok() && usize::try_from(tmp_x).is_ok() {
        new_y = usize::try_from(tmp_y).unwrap();
        new_x = usize::try_from(tmp_x).unwrap();
    } else {
        println!("Failed usize conv");
        return None
    };

    println!("new pos: [{},{}]", new_y, new_x);

    if new_y >= xmas_matrix.len()
    || new_x >= xmas_matrix[new_y].len()
    || xmas_matrix[new_y][new_x] != xmas[step] {
        return None;
    }

    return Some([new_y, new_x]);
} 
