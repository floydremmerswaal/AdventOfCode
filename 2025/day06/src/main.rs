use regex::Regex;
use std::{fs, process::exit};

fn column_times(vecs: &Vec<Vec<u64>>, index: usize) -> u64 {
    let mut total = 1;
    for line in 0..vecs.len() {
        total *= vecs[line][index];
    }
    return total;
}

fn column_add(vecs: &Vec<Vec<u64>>, index: usize) -> u64 {
    let mut total = 0;
    for line in 0..vecs.len() {
        total += vecs[line][index];
    }
    return total;
}

fn process_column(vecs: &Vec<u64>) {
    let top_length = vecs[0].to_string().len();
    let bot_length = vecs[vecs.len() - 1].to_string().len();
    if (top_length > bot_length) {
        let mut numbers: Vec<u64> = vec![0; top_length];

        for number in vecs {
            let str = number.to_string();
            let length = str.len();
            for digit in str {
                numbers[]
            }
        }
    }
}

fn main() {
    let content =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file");

    let lines = content.lines();
    let length = lines.count();

    let mut numbers: Vec<Vec<u64>> = vec![];

    let mut count = 0;

    for (i, line) in content.lines().into_iter().enumerate() {
        if i == length - 1 {
            for x in &numbers {
                for y in x {
                    print!("{} ", y);
                }
                println!("");
            }

            for (idx, op) in line.split_whitespace().enumerate() {
                if op == "*" {
                    count += column_times(&numbers, idx)
                } else if op == "+" {
                    count += column_add(&numbers, idx)
                } else {
                    exit(-1);
                }
            }

            break;
        }

        numbers.push(vec![]);

        for no in line.split_whitespace() {
            numbers[i].push(no.parse::<u64>().unwrap());
        }
    }

    println!("Solution to part 1: {}", count);
}
