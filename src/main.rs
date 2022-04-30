use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Debug)]
struct _Vertex {
    x: u32,
    y: u32,
}

fn load_day03(filename: &str) -> (_Vertex, HashSet<_Vertex>) {
    let input = fs::read_to_string(filename).unwrap();
    let mut cordinate: HashSet<_Vertex> = HashSet::new();
    let mut x_length: Option<u8> = None;
    let mut y_length = 0;
    for (y, straight) in input.lines().enumerate() {
        if x_length.is_none() {
            let i = straight.len() as u8;
            x_length = Some(i);
        }
        for (x, target) in straight.chars().enumerate() {
            if target != '#' {
                continue;
            }
            let i = _Vertex {
                x: x as u32,
                y: y as u32,
            };
            cordinate.insert(i);
        }
        y_length += 1;
    }
    let length = _Vertex {
        x: x_length.unwrap() as u32,
        y: y_length,
    };
    (length, cordinate)
}

fn travel(right: u32, down: u32, limit: &_Vertex, input: &HashSet<_Vertex>) -> u64 {
    let mut count = 0;
    for i in 1..limit.y {
        let x = i * right % limit.x;
        let y = i * down;
        if y > limit.y {break;}
        let cord = _Vertex{x,y};
        if input.contains(&cord) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{:?}", Day01::new(".\\input\\01.txt"));
    println!("{:?}", Day02::new(".\\input\\02.txt"));

    let (length, input) = load_day03(".\\input\\03.txt");
    let mut count = 0;
    for y in 1..length.y {
        let x = y * 3 % length.x;
        let cord = _Vertex{x,y};
        if input.contains(&cord) {
            count += 1;
        }
    }
    println!("Day 03 Part One: {}", count);
    let mut count = travel(1, 1, &length, &input);
    count *= travel(3, 1, &length, &input);
    count *= travel(5, 1, &length, &input);
    count *= travel(7, 1, &length, &input);
    count *= travel(1, 2, &length, &input);
    println!("Day Test {}", count);
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

#[derive(Debug)]
struct _Pass {
    min: u8,
    max: u8,
    target: char,
    pass: String,
}

#[derive(Debug)]
struct Day02(i64, i64);

impl Day02 {
    fn new(filename: &str) -> Day02 {
        let input = Day02::_load(filename);
        Day02(Day02::_part_one(&input), Day02::_part_two(&input))
    }

    fn _load(filename: &str) -> Vec<_Pass> {
        let re = Regex::new(r"(?P<min>\d+).(?P<max>\d+).(?P<char>\w)..(?P<pass>\w+)").unwrap();
        let input = fs::read_to_string(filename).unwrap();
        let mut output: Vec<_Pass> = Vec::new();

        for i in input.lines() {
            let cap = re.captures(i);
            let cap = match cap {
                None => continue,
                Some(x) => x,
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

    fn _part_one(input: &[_Pass]) -> i64 {
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

    fn _part_two(input: &[_Pass]) -> i64 {
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
}
