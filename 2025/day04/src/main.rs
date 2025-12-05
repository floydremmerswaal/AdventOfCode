use std::fs;

fn one_if_at(matrix: &Vec<Vec<char>>, i: i32, j: i32, height: usize, width: usize) -> u32 {
    let h = height as i32;
    let w = width as i32;
    let s_i = i as usize;
    let s_j = j as usize;

    // println!("checking {}, {}", i, j);
    if i < 0 || i >= h || j < 0 || j >= w {
        // println!("out of bounds");
        return 0;
    }
    // println!("Char is {}", matrix[s_i][s_j]);
    if matrix[s_i][s_j] == '@' {
        // println!("found");
        return 1;
    }
    // println!("not found");
    return 0;
}

fn count_surrounding(
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> u32 {
    let mut count = 0;
    for x in 0..=2 {
        for y in 0..=2 {
            let i_idx: i32 = (i as i32) + x - 1;
            let j_idx: i32 = (j as i32) + y - 1;
            count += one_if_at(matrix, i_idx, j_idx, height, width);
        }
    }
    count -= one_if_at(matrix, i as i32, j as i32, height, width);
    return count;
}

fn can_access(matrix: &Vec<Vec<char>>, i: usize, j: usize, height: usize, width: usize) -> bool {
    let surrounding_count = count_surrounding(matrix, i, j, height, width);
    return surrounding_count < 4;
}

fn _debug_print(matrix: &Vec<Vec<char>>) {
    let height = matrix.len();
    let width = matrix[0].len();
    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            print!("{}", matrix[i][j]);
        }
        println!();
    }

    println!();

    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            print!("{}", one_if_at(&matrix, i as i32, j as i32, height, width));
        }
        println!();
    }

    println!();

    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            print!("{}", count_surrounding(&matrix, i, j, height, width));
        }
        println!();
    }

    println!();

    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            if matrix[i][j] == '.' {
                print!(".");
                continue;
            }
            if count_surrounding(&matrix, i, j, height, width) < 4 {
                print!("X");
            } else {
                print!("@");
            }
        }
        println!();
    }
    println!();

    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            if matrix[i][j] == '.' {
                print!(".");
                continue;
            }
            if can_access(&matrix, i, j, height, width) {
                print!("X");
            } else {
                print!("@");
            }
        }
        println!();
    }
}

fn main() {
    let content =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<char>> = vec![];

    for (i, line) in content.lines().enumerate() {
        matrix.push(vec![]);
        for char in line.chars() {
            matrix[i].push(char);
        }
    }

    let height = matrix.len();
    let width = matrix[0].len();

    let mut count1 = 0;

    for (i, _line) in matrix.iter().enumerate() {
        for (j, _elem) in matrix[i].iter().enumerate() {
            if matrix[i][j] == '.' {
                continue;
            }

            if can_access(&matrix, i, j, height, width) {
                count1 += 1;
            }
        }
    }

    println!("Solution to part 1 = {}", count1);

    // part 2

    let mut count2 = 0;
    loop {
        let mut vecs: Vec<(usize, usize)> = vec![];
        for (i, _line) in matrix.iter().enumerate() {
            for (j, _elem) in matrix[i].iter().enumerate() {
                if matrix[i][j] == '.' {
                    continue;
                }
                if can_access(&matrix, i, j, height, width) {
                    vecs.push((i, j));
                }
            }
        }
        if vecs.len() == 0 {
            break;
        }

        for (i, j) in vecs {
            count2 += 1;
            matrix[i][j] = '.';
        }
    }

    println!("Solution to part 2 = {}", count2);
}
