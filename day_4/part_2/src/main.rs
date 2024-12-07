use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ms: [char; 2] = ['M','S'];
const directions: [[i32; 2]; 4] = [
    [1,1],   // se   
    [1,-1],  // sw
    [-1,-1], // nw
    [-1,1]   // ne
];

fn main() {
    const file_name: &str = "../sample-input.txt";
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
                count += check_for_cross_mas(&xmas_matrix, [i,j]);
            }
        }
    }
    println!("Amount of xmas: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn check_for_cross_mas(xmas_matrix: &Vec<Vec<char>>, pos: [usize; 2]) -> u32 {
    let cross_mas: bool = false;

    // Check cross shape with a as center...
    // top left to bottom right:
    let tmp_nw: [i32; 2] = [pos[0] as i32 - 1, pos[1] as i32 - 1];
    let tmp_se: [i32; 2] = [pos[0] as i32 + 1, pos[1] as i32 + 1];

    // top right to bottom left
    let tmp_ne: [i32; 2] = [pos[0] as i32 - 1, pos[1] as i32 + 1];
    let tmp_sw: [i32; 2] = [pos[0] as i32 + 1, pos[1] as i32 - 1];

    let nw: [usize; 2];
    let se: [usize; 2];

    let ne: [usize; 2];
    let sw: [usize; 2];

    if
        usize::try_from(tmp_nw[0]).is_ok() && usize::try_from(tmp_nw[1]).is_ok() &&
        usize::try_from(tmp_se[0]).is_ok() && usize::try_from(tmp_se[1]).is_ok() &&
        usize::try_from(tmp_ne[0]).is_ok() && usize::try_from(tmp_ne[1]).is_ok() &&
        usize::try_from(tmp_sw[0]).is_ok() && usize::try_from(tmp_sw[1]).is_ok()
    {
        nw = [usize::try_from(tmp_nw[0]).unwrap(), usize::try_from(tmp_nw[1]).unwrap()];
        se = [usize::try_from(tmp_se[0]).unwrap(), usize::try_from(tmp_se[1]).unwrap()];
        ne = [usize::try_from(tmp_ne[0]).unwrap(), usize::try_from(tmp_ne[1]).unwrap()];
        sw = [usize::try_from(tmp_sw[0]).unwrap(), usize::try_from(tmp_sw[1]).unwrap()];
    } else {
        println!("Failed usize conv");
        return 0;
    };

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
        return 1;
    }

    return 0;
}
