use std::collections::{HashMap, HashSet};

use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1(lines: &Vec<String>) {
    let width = lines[0].len();

    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits = 0;

    let first = lines[0].find("S").unwrap();
    beams.insert(first);

    for i in 1..lines.len() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        let locs = lines[i].match_indices("^");

        for (loc, _char) in locs {
            if beams.contains(&loc) {
                splits += 1;
                if loc != 0 {
                    new_beams.insert(loc - 1);
                }
                if loc != width - 1 {
                    new_beams.insert(loc + 1);
                }
                beams.remove(&loc);
            }
        }

        beams.extend(&new_beams);
    }

    println!("Solution to part 1: {}", splits);
}

fn count_from_loc(
    lines: &Vec<String>,
    x: usize,
    y: usize,
    known_values: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if known_values.contains_key(&(x, y)) {
        return *known_values.get(&(x, y)).unwrap();
    }

    for i in x..lines.len() {
        if lines[i].as_bytes()[y] == b'^' {
            let mut left = 0;
            let mut right = 0;
            if y > 0 {
                left = count_from_loc(lines, i, y - 1, known_values);
            }
            if y < lines[0].len() {
                right = count_from_loc(lines, i, y + 1, known_values);
            }
            known_values.insert((x, y), left + right);
            return left + right;
        }
    }

    known_values.insert((x, y), 1);
    return 1;
}

fn main() {
    let lines = lines_from_file("input/input.txt");
    part_1(&lines);

    let first = lines[0].find("S").unwrap();

    let mut known_values: HashMap<(usize, usize), u64> = HashMap::new();

    let part2 = count_from_loc(&lines, 0, first, &mut known_values);

    println!("Solution to part 2: {}", part2);
}
