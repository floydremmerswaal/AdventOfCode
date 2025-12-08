use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

use std::collections::HashSet;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn distance(p1: (usize, usize, usize), p2: (usize, usize, usize)) -> f32 {
    let x1 = p1.0 as i64;
    let x2 = p2.0 as i64;
    let y1 = p1.1 as i64;
    let y2 = p2.1 as i64;
    let z1 = p1.2 as i64;
    let z2 = p2.2 as i64;

    let d1 = (x1 - x2).pow(2);
    let d2 = (y1 - y2).pow(2);
    let d3 = (z1 - z2).pow(2);

    return f32::sqrt((d1 as f32) + (d2 as f32) + (d3 as f32));
}

fn part_1(top_n: &[((usize, usize, usize), (usize, usize, usize), f32)]) {
    let mut sets: Vec<HashSet<(usize, usize, usize)>> = vec![];
    let mut is_connected: HashSet<(usize, usize, usize)> = HashSet::new();

    for x in top_n {
        let con1 = is_connected.contains(&x.0);
        let con2 = is_connected.contains(&x.1);
        if !con1 && !con2 {
            // handle this case;
            sets.push(HashSet::from([x.0, x.1]));
        } else if con1 && con2 {
            let idx_1 = sets.iter().position(|set| set.contains(&x.0)).unwrap();
            let idx_2 = sets.iter().position(|set| set.contains(&x.1)).unwrap();

            if idx_1 == idx_2 {
                continue;
            }

            let set_2 = sets[idx_2].clone();

            sets[idx_1].extend(set_2);

            sets.remove(idx_2);
        } else {
            let mut l = x.0;
            let mut r = x.1;
            if con2 {
                let x = l;
                l = r;
                r = x;
            }

            let idx = sets.iter().position(|set| set.contains(&l)).unwrap();

            let set = &mut sets[idx];
            set.insert(r);
        }

        is_connected.insert(x.0);
        is_connected.insert(x.1);
    }

    sets.sort_by_key(|x| x.len());

    let mut result = 1;

    for i in 0..3 {
        result *= sets[sets.len() - i - 1].len();
    }

    println!("Solution to part 1: {}", result);
}

fn part_2(connections: &[((usize, usize, usize), (usize, usize, usize), f32)], no_points: usize) {
    let mut sets: Vec<HashSet<(usize, usize, usize)>> = vec![];
    let mut is_connected: HashSet<(usize, usize, usize)> = HashSet::new();

    let mut result = 0;

    for x in connections {
        let con1 = is_connected.contains(&x.0);
        let con2 = is_connected.contains(&x.1);
        if !con1 && !con2 {
            sets.push(HashSet::from([x.0, x.1]));
        } else if con1 && con2 {
            let idx_1 = sets.iter().position(|set| set.contains(&x.0)).unwrap();
            let idx_2 = sets.iter().position(|set| set.contains(&x.1)).unwrap();

            if idx_1 == idx_2 {
                continue;
            }

            let set_2 = sets[idx_2].clone();

            sets[idx_1].extend(set_2);

            sets.remove(idx_2);
        } else {
            let mut l = x.0;
            let mut r = x.1;
            if con2 {
                let x = l;
                l = r;
                r = x;
            }

            let idx = sets.iter().position(|set| set.contains(&l)).unwrap();

            let set = &mut sets[idx];
            set.insert(r);
        }

        is_connected.insert(x.0);
        is_connected.insert(x.1);

        if sets.len() == 1 && sets[0].len() == no_points {
            result = x.0.0 * x.1.0;
            break;
        }
    }

    println!("Result for part 2: {}", result);
}

fn main() {
    let lines = lines_from_file("input/input.txt");
    let count = lines.len();

    let mut points: Vec<(usize, usize, usize)> = vec![];

    for line in &lines {
        let mut temp: Vec<usize> = vec![];
        for num in line.split(',') {
            temp.push(num.parse::<usize>().unwrap());
        }

        points.push((temp[0], temp[1], temp[2]));
    }

    let mut dist_list: Vec<((usize, usize, usize), (usize, usize, usize), f32)> = vec![];

    for idx_1 in 0..points.len() {
        for idx_2 in 1 + idx_1..points.len() {
            let point_1 = points[idx_1];
            let point_2 = points[idx_2];

            let dist = distance(point_1, point_2);
            dist_list.push((point_1, point_2, dist));
        }
    }

    dist_list.sort_by(|x, y| x.2.partial_cmp(&y.2).unwrap());

    part_1(&dist_list[0..1000]);

    part_2(&dist_list[..], count);
}
