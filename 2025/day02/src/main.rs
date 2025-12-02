use std::fs;

fn check_condition_1(n: u64) -> bool {
    let s = n.to_string();
    let l = s.chars().count();

    if l % 2 != 0 {
        return false;
    }

    let split = s.split_at(l / 2);
    let left = split.0;
    let right = split.1;

    if left == right {
        return true;
    }

    return false;
}

fn check_condition_2(n: usize) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let length = bytes.len();

    for i in 1..=length / 2 {
        if length % i != 0 {
            continue;
        }

        let first_slice = &bytes[0..i];
        let mut found = true;

        for j in 1..length / i {
            let start = i * j;
            let end = start + i;
            let slice = &bytes[start..end];

            if first_slice != slice {
                found = false;
                break;
            }
        }

        if found {
            return true;
        }
    }
    return false;
}

fn main() {
    let content =
        fs::read_to_string("input/input02.txt").expect("Should have been able to read the file");

    let mut count1 = 0;
    let mut count2 = 0;

    let ranges = content.split(',');
    for range in ranges {
        let mut numbers = range.split('-');
        let left = numbers.next().unwrap().parse::<u64>().unwrap();
        let right = numbers.next().unwrap().parse::<u64>().unwrap();
        for i in left..=right {
            if check_condition_1(i) {
                count1 += i;
            }
            if check_condition_2(i.try_into().unwrap()) {
                count2 += i;
            }
        }
    }
    println!("Problem 1 solution: {}", count1);
    println!("Problem 2 solution: {}", count2);
}
