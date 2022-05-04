use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs;

use aoc2020_rust::Day;

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

fn parse_year(input: &str, min: i32, max: i32) -> bool {
    if input.len() != 4 {
        return false;
    }
    let year = input.parse::<i32>();
    if year.is_err() {
        return false;
    }
    let year = year.unwrap();
    min <= year && year <= max
}

#[derive(Debug)]
enum _Height {
    Cm(i32),
    Inch(i32),
}

impl _Height {
    fn parse(input: &str) -> Option<_Height> {
        let re = Regex::new(r"(?P<num>\d+)(?P<type>\S{2})").unwrap();
        let capture = re.captures(input);
        capture.as_ref()?;
        let capture = capture.unwrap();

        let num = capture["num"].parse::<i32>();
        if num.is_err() {
            return None;
        }
        let num = num.unwrap();

        let h_type = capture["type"].to_string();
        match h_type.as_str() {
            "cm" => Some(_Height::Cm(num)),
            "in" => Some(_Height::Inch(num)),
            _ => None,
        }
    }
}

fn check_valid(id: &_PassportId) -> bool {
    match id {
        _PassportId::Byr(x) => parse_year(x, 1920, 2002),
        _PassportId::Iyr(x) => parse_year(x, 2010, 2020),
        _PassportId::Eyr(x) => parse_year(x, 2020, 2030),
        _PassportId::Hgt(x) => {
            let height = _Height::parse(x);
            if height.is_none() {
                return false;
            }
            let height = height.unwrap();
            match height {
                _Height::Cm(x) => (150..=193).contains(&x),
                _Height::Inch(x) => (59..=76).contains(&x),
            }
        }
        _PassportId::Hcl(x) => {
            let re = Regex::new(r"\#(?P<hexs>[0-9a-f]+)").unwrap();
            let cap = re.captures(x);
            if cap.is_none() {
                return false;
            }
            let cap = cap.unwrap();
            let cap = cap["hexs"].to_string();
            cap.len() == 6
        }
        _PassportId::Ecl(x) => matches!(
            x.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ),
        _PassportId::Pid(x) => {
            let re = Regex::new(r"(?P<id>\d+)").unwrap();
            let cap = re.captures(x);
            let cap = cap.unwrap();
            let cap = cap["id"].to_string();
            cap.len() == 9
        }
        _PassportId::Cid(_) => true,
    }
}

fn part_one(input: &[Option<_PassportId>], check_field: bool) -> u32 {
    let mut is_cid = false;
    let mut valid = 0;
    let mut current = 0;
    let mut skip_until_none = false;
    for i in input.iter() {
        match i {
            Some(x) => {
                if skip_until_none {
                    continue;
                }
                let accept = check_valid(x);
                if !accept && check_field {
                    skip_until_none = true;
                    continue;
                }
                current += 1;
                if let _PassportId::Cid(_) = x {
                    is_cid = true;
                };
            }
            None => {
                if !is_cid && current == 7 || current == 8 && !skip_until_none {
                    valid += 1;
                }
                is_cid = false;
                current = 0;
                skip_until_none = false;
            }
        }
    }
    valid
}

fn main() {
    println!("{:?}", Day::new(1, ".\\input\\01.txt").unwrap());
    println!("{:?}", Day::new(2, ".\\input\\02.txt").unwrap());

    println!("{:?}", Day03::new(".\\input\\03.txt"));

    let id_list = read_passport_id(".\\input\\04.txt");
    println!("Valid {}", part_one(&id_list, false));
    println!("Part 2: {}", part_one(&id_list, true));
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
