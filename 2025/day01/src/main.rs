use std::{fs, process::exit};

fn main() {
    let content =
        fs::read_to_string("../input/input01.txt").expect("Should have been able to read the file");

    let mut total: i32 = 50;
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;

    let lines = content.lines();

    for line in lines {
        let value: i32 = line[1..].parse::<i32>().unwrap();
        let char = line.chars().nth(0).unwrap_or('X');

        let prev: i32 = total;

        if char == 'X' {
            exit(-1)
        } else if char == 'R' {
            total += value;

            count2 += total / 100;
        } else {
            total -= value;
            if prev == 0 {
                count2 += (((100 - prev) + value) / 100) - 1;
            } else {
                count2 += ((100 - prev) + value) / 100;
            }
        }

        total = total.rem_euclid(100);

        if total == 0 {
            count1 += 1;
        }
    }

    println!("Answer 1: {}", count1);
    println!("Answer 2: {}", count2);
}
