use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum _PassportId {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String),
}

fn passport_match(capture: &Captures) -> Option<_PassportId> {
    let x = capture["type"].to_string();
    let data = capture["data"].to_string();
    match x.as_str() {
        "byr" => Some(_PassportId::Byr(data)),
        "iyr" => Some(_PassportId::Iyr(data)),
        "eyr" => Some(_PassportId::Eyr(data)),
        "hgt" => Some(_PassportId::Hgt(data)),
        "hcl" => Some(_PassportId::Hcl(data)),
        "ecl" => Some(_PassportId::Ecl(data)),
        "pid" => Some(_PassportId::Pid(data)),
        "cid" => Some(_PassportId::Cid(data)),
        _ => None,
    }
}

fn read_passport_id(filename: &str) -> Vec<Option<_PassportId>> {
    let input = fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"(?P<type>\w{3}).(?P<data>\S+)").unwrap();
    let mut passport_id_list: Vec<Option<_PassportId>> = Vec::new();

    for (n, i) in input.lines().enumerate() {
        if i.is_empty() {
            passport_id_list.push(None);
        }
        for j in re.captures_iter(i) {
            let x = passport_match(&j);
            if x.is_none() {
                panic!("Input error at line {}, no match.\nLines: {}", n, i);
            }
            passport_id_list.push(x);
        }
    }
    if passport_id_list[passport_id_list.len() - 1].is_some() {
        passport_id_list.push(None);
    }
    passport_id_list
}

fn part_one(input: &[Option<_PassportId>]) -> u32 {
    let mut is_cid = false;
    let mut valid = 0;
    let mut current = 0;
    for i in input.iter() {
        match i {
            Some(x) => {
                current += 1;
                if let _PassportId::Cid(_) = x {
                    is_cid = true;
                };
            }
            None => {
                if !is_cid && current == 7 || current == 8 {
                    valid += 1;
                }
                is_cid = false;
                current = 0;
            }
        }
    }
    valid
}

fn main() {
    println!("{:?}", Day01::new(".\\input\\01.txt"));
    println!("{:?}", Day02::new(".\\input\\02.txt"));
    println!("{:?}", Day03::new(".\\input\\03.txt"));

    let id_list = read_passport_id(".\\input\\04.txt");
    println!("Valid {}", part_one(&id_list));
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct _Vertex {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Day03(u64, u64);

impl Day03 {
    fn new(filename: &str) -> Day03 {
        let (limit, input) = Day03::_load(filename);
        let part_one = Day03::_travel(3, 1, &limit, &input);

        let mut part_two = Day03::_travel(1, 1, &limit, &input);
        part_two *= Day03::_travel(3, 1, &limit, &input);
        part_two *= Day03::_travel(5, 1, &limit, &input);
        part_two *= Day03::_travel(7, 1, &limit, &input);
        part_two *= Day03::_travel(1, 2, &limit, &input);

        Day03(part_one, part_two)
    }

    fn _load(filename: &str) -> (_Vertex, HashSet<_Vertex>) {
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

    fn _travel(right: u32, down: u32, limit: &_Vertex, input: &HashSet<_Vertex>) -> u64 {
        let mut count = 0;
        for i in 1..limit.y {
            let x = i * right % limit.x;
            let y = i * down;
            if y > limit.y {
                break;
            }
            let cord = _Vertex { x, y };
            if input.contains(&cord) {
                count += 1;
            }
        }
        count
    }
}
