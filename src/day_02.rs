use crate::Day;
use regex::Regex;

#[derive(Debug)]
struct Pass {
    min: u8,
    max: u8,
    target: char,
    pass: String,
}

pub fn parse(raw_input: &str) -> Day {
    let input = load(raw_input);
    Day {
        day: 2,
        part_one: part_one(&input).to_string(),
        part_two: part_two(&input).to_string(),
    }
}

fn load(raw_input: &str) -> Vec<Pass> {
    let re = Regex::new(r"(?P<min>\d+).(?P<max>\d+).(?P<char>\w)..(?P<pass>\w+)").unwrap();
    let mut output: Vec<Pass> = Vec::new();

    for i in raw_input.lines() {
        let cap = re.captures(i);
        let cap = match cap {
            None => continue,
            Some(x) => x,
        };
        let pass_char = cap["char"].chars().next().unwrap();
        let result: Pass = Pass {
            min: cap["min"].parse().unwrap(),
            max: cap["max"].parse().unwrap(),
            target: pass_char,
            pass: cap["pass"].to_string(),
        };
        output.push(result);
    }

    output
}

fn part_one(input: &[Pass]) -> i64 {
    let mut valid = 0;
    for i in input.iter() {
        let mut count = i.pass.clone();
        count.retain(|x| x == i.target);
        let count = count.len() as u8;
        if i.min <= count && count <= i.max {
            valid += 1;
        };
    }
    valid
}

fn part_two(input: &[Pass]) -> i64 {
    let mut valid = 0;
    for i in input.iter() {
        let pass_char: char = i.pass.chars().nth((i.min - 1).into()).unwrap();
        let mut count = 0;
        if pass_char == i.target {
            count += 1;
        }
        let pass_char: char = i.pass.chars().nth((i.max - 1).into()).unwrap();
        if pass_char == i.target {
            count += 1;
        }
        if count == 1 {
            valid += 1;
        };
    }
    valid
}
