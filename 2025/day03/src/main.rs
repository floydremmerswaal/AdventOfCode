use itertools::Itertools;
use std::fs;

fn get_leftmost_max_and_index(vec: Vec<u32>) -> (usize, u32) {
    let i_r = vec.iter().rev().position_max().unwrap();
    let i = vec.len() - i_r - 1;
    let v = vec[i];
    return (i, v);
}

fn process_line_1(str: &str) -> u32 {
    let numbers: Vec<u32> = str
        .chars()
        .map(|c| c.to_digit(10).expect("conversion error"))
        .collect();
    let (li, lv) = get_leftmost_max_and_index(numbers[0..numbers.len() - 1].to_vec());
    let (_ri, rv) = get_leftmost_max_and_index(numbers[li + 1..numbers.len()].to_vec());

    return 10 * lv + rv;
}

fn process_line_2(str: &str) -> u64 {
    println!("{}", str);
    let numbers: Vec<u32> = str
        .chars()
        .map(|c| c.to_digit(10).expect("conversion error"))
        .collect();
    let mut total = 0;

    let mut bound_l = 0;
    for i in (0..=11).rev() {
        let (idx, v) = get_leftmost_max_and_index(numbers[bound_l..numbers.len() - i].to_vec());
        let base: u64 = 10u64.pow(i as u32);
        total += base * v as u64;

        bound_l += idx + 1;
    }

    println!("Total: {}", total);
    return total;
}

fn main() {
    let content =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file");

    let mut count1: u32 = 0;
    let mut count2: u64 = 0;

    for line in content.lines() {
        count1 += process_line_1(line);
        count2 += process_line_2(line);
    }

    println!("Solution part 1: {}", count1);
    println!("Solution part 2: {}", count2);
}
