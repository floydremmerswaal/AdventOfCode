use std::collections::HashSet;
use std::fs;

fn lookup(list: &[(u64, u64)], value: &u64) -> bool {
    for (l, r) in list {
        if l <= value && value <= r {
            return true;
        }
    }
    return false;
}

fn merge_possible(list: &Vec[(u64, u64)]){
    if list.len() == 1 {
        return list;
    }

    let options 
}

fn main() {
    let content =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file");

    let mut fresh_list: Vec<(u64, u64)> = vec![];
    let mut fresh_set = HashSet::new();
    let mut fresh_count = 0;
    for line in content.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let mut split = line.split('-');

            let left = split.next().unwrap().parse::<u64>().unwrap();
            let right = split.next().unwrap().parse::<u64>().unwrap();
            fresh_list.push((left, right));

            for i in left..=right {
                fresh_set.insert(i);
            }
            continue;
        }

        let value = line.parse::<u64>().unwrap();
        if lookup(fresh_list.as_slice(), &value) {
            fresh_count += 1;
        }
    }

    println!("Solution to day 1: {}", fresh_count);
    println!("Solution to day 2: {}", fresh_set.len());
}
