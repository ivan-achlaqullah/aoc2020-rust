use crate::Day;
use std::collections::HashSet;

pub fn parse(raw_input: &str) -> Day {
    let input = {
        let mut nums: HashSet<i64> = HashSet::new();
        for i in raw_input.lines() {
            nums.insert(i.parse().unwrap());
        }
        nums
    };

    let r1 = {
        let (x, y) = find_two(&input, 2020).unwrap();
        x * y
    };

    let r2 = {
        let (x, y, z) = find_three(&input, 2020).unwrap();
        x * y * z
    };

    Day {
        day: 1,
        part_one: r1.to_string(),
        part_two: r2.to_string(),
    }
}

fn find_two(inputs: &HashSet<i64>, max: i64) -> Option<(i64, i64)> {
    for i in inputs.iter() {
        let j = max - i;
        if inputs.contains(&j) {
            return Some((*i, j));
        }
    }
    None
}

fn find_three(inputs: &HashSet<i64>, max: i64) -> Option<(i64, i64, i64)> {
    for i in inputs.iter() {
        let extra = max - i;
        let extra = find_two(inputs, extra);
        match extra {
            None => continue,
            Some((j, k)) => return Some((*i, j, k)),
        }
    }
    None
}
