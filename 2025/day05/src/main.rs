use std::fs;

fn lookup(list: &[(u64, u64)], value: &u64) -> bool {
    for (l, r) in list {
        if l <= value && value <= r {
            return true;
        }
    }
    return false;
}

fn can_merge(left: (u64, u64), right: (u64, u64)) -> bool {
    let x1 = left.0;
    let x2 = right.0;
    let y1 = left.1;
    let y2 = right.1;

    if y1 < x2 {
        return false;
    }

    if y2 < x1 {
        return false;
    }

    return true;
}

fn merge(left: (u64, u64), right: (u64, u64)) -> (u64, u64) {
    assert!(can_merge(left, right));

    let left_most;
    let right_most;

    if left.0 < right.0 {
        left_most = left.0;
    } else {
        left_most = right.0;
    }

    if left.1 > right.1 {
        right_most = left.1;
    } else {
        right_most = right.1;
    }

    return (left_most, right_most);
}

fn merge_or_add(list: &mut Vec<(u64, u64)>, elem: (u64, u64)) {
    for i in 0..list.len() {
        if can_merge(list[i], elem) {
            list[i] = merge(list[i], elem);
            return;
        }
    }

    list.push(elem);
}

fn merge_possible(list: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let len = list.len();

    if len == 1 {
        return list.to_vec();
    }

    let mut new_list = vec![];

    for elem in list {
        merge_or_add(&mut new_list, *elem);
    }

    return new_list;
}

fn main() {
    let content =
        fs::read_to_string("input/input.txt").expect("Should have been able to read the file");

    let mut fresh_list: Vec<(u64, u64)> = vec![];
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

            continue;
        }

        let value = line.parse::<u64>().unwrap();
        if lookup(&fresh_list, &value) {
            fresh_count += 1;
        }
    }

    loop {
        let size = fresh_list.len();

        let new_list = merge_possible(&mut fresh_list);

        if size == new_list.len() {
            break;
        }

        fresh_list = new_list;
    }

    let mut total = 0;
    for (x, y) in fresh_list {
        total += y - x + 1;
    }

    println!("Solution to day 1: {}", fresh_count);
    println!("Solution to day 2: {}", total);
}
