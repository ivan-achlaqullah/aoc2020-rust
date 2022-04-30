use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    println!("{:?}", Day01::new(".\\input\\01.txt"));
    let re = Regex::new(r"(?P<min>\d+).(?P<max>\d+).(?P<char>\w)..(?P<pass>\w+)").unwrap();
    let input = fs::read_to_string(".\\input\\02.txt").unwrap();
    for (i,j) in input.lines().enumerate() {
        let cap = re.captures(j);
        let cap = match cap {
            None => continue,
            Some(x) => x
        };
        println!("Lines {}", i);
        println!("{}",j);
        println!("{}, {}, {}, {}",
            &cap["min"],
            &cap["max"],
            &cap["char"],
            &cap["pass"]
        );
    }
}

#[derive(Debug)]
struct Day01(i64, i64);

impl Day01 {
    fn new(filename: &str) -> Day01 {
        let input = fs::read_to_string(filename).unwrap();
        let input = {
            let mut nums: HashSet<i64> = HashSet::new();
            for i in input.lines() {
                nums.insert(i.parse().unwrap());
            }
            nums
        };

        let r1 = {
            let (x, y) = Day01::find_two(&input, 2020).unwrap();
            x * y
        };

        let r2 = {
            let (x, y, z) = Day01::find_three(&input, 2020).unwrap();
            x * y * z
        };

        Day01(r1, r2)
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
            let extra = Day01::find_two(inputs, extra);
            match extra {
                None => continue,
                Some((j, k)) => return Some((*i, j, k)),
            }
        }
        None
    }
}
