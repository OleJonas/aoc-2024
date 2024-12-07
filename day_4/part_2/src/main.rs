use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::convert::TryFrom;
use std::num::TryFromIntError;

const ms: [char; 2] = ['M','S'];
const directions: [[i32; 2]; 4] = [
    [1,1],   // se   
    [1,-1],  // sw
    [-1,-1], // nw
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
    // Do 1 -1 here to not go to outer edges and get out of bounds when checking for cross
    for i in 1..xmas_matrix.len()-1 {
        for j in 1..xmas_matrix[i].len()-1 {
            if xmas_matrix[i][j] == 'A' {
                let cross_mas = check_for_cross_mas(&xmas_matrix, [i,j]);
                if cross_mas.is_ok() {
                    count += cross_mas.unwrap();
                }
            }
        }
    }
    println!("Amount of xmas: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_for_cross_mas(xmas_matrix: &Vec<Vec<char>>, pos: [usize; 2]) -> Result<u32, TryFromIntError> {
    let nw: [usize; 2] = [
        usize::try_from(pos[0] as i32 - 1)?,
        usize::try_from(pos[1] as i32 - 1)?
    ];
    let se: [usize; 2] = [
        usize::try_from(pos[0] as i32 + 1)?,
        usize::try_from(pos[1] as i32 + 1)?
    ];
    let ne: [usize; 2] = [
        usize::try_from(pos[0] as i32 - 1)?,
        usize::try_from(pos[1] as i32 + 1)?
    ];
    let sw: [usize; 2] = [
        usize::try_from(pos[0] as i32 + 1)?,
        usize::try_from(pos[1] as i32 - 1)?
    ];

    let cross_mas: bool = false;
    if 
        // Check top left to bottom right
        xmas_matrix[nw[0]][nw[1]] != 'A' &&
        xmas_matrix[se[0]][se[1]] != 'A' &&
        xmas_matrix[nw[0]][nw[1]] != xmas_matrix[se[0]][se[1]] &&
        ms.contains(&xmas_matrix[nw[0]][nw[1]]) &&
        ms.contains(&xmas_matrix[se[0]][se[1]]) &&

        // Then top right to bottom left
        xmas_matrix[ne[0]][ne[1]] != 'A' &&
        xmas_matrix[sw[0]][sw[1]] != 'A' &&
        xmas_matrix[ne[0]][ne[1]] != xmas_matrix[sw[0]][sw[1]] &&
        ms.contains(&xmas_matrix[ne[0]][ne[1]]) &&
        ms.contains(&xmas_matrix[sw[0]][sw[1]])
    {
        return Ok(1);
    }

    return Ok(0);
}
