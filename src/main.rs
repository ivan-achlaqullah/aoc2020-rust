use std::fs;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct _Pass {
    min: u8,
    max: u8,
    target: char,
    pass: String,
}

fn load_day_02(filename: &str) -> Vec<_Pass> {
    let re = Regex::new(r"(?P<min>\d+).(?P<max>\d+).(?P<char>\w)..(?P<pass>\w+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let mut output: Vec<_Pass> = Vec::new();

    for i in input.lines() {
        let cap = re.captures(i);
        let cap = match cap {
            None => continue,
            Some(x) => x
        };
        let pass_char = cap["char"].chars().next().unwrap();
        let result: _Pass = _Pass {
            min: cap["min"].parse().unwrap(),
            max: cap["max"].parse().unwrap(),
            target: pass_char,
            pass: cap["pass"].to_string(),
        };
        output.push(result);
    }

    output
}

fn main() {
    println!("{:?}", Day01::new(".\\input\\01.txt"));

    let input = load_day_02(".\\input\\02.txt");
    println!("Regex Parse: {:#?}", &input[0]);
    println!("Total: {}", &input.len());

    let mut valid = 0;
    for i in input.iter() {
        let mut count = i.pass.clone();
        count.retain(|x| x == i.target);
        let count = count.len() as u8;
        if i.min <= count && count <= i.max {valid += 1;};
    }
    println!("Valid (Part One): {}", &valid);
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
