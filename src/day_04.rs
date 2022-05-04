use regex::{Captures, Regex};

use crate::Day;

pub fn parse(raw_input: &str) -> Day {
    let input = read_passport_id(raw_input);
    Day {
        day: 4,
        part_one: count_passports(&input, false).to_string(),
        part_two: count_passports(&input, true).to_string(),
    }
}

#[derive(Debug)]
enum PassportId {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String),
}

fn passport_match(capture: &Captures) -> Option<PassportId> {
    let x = capture["type"].to_string();
    let data = capture["data"].to_string();
    match x.as_str() {
        "byr" => Some(PassportId::Byr(data)),
        "iyr" => Some(PassportId::Iyr(data)),
        "eyr" => Some(PassportId::Eyr(data)),
        "hgt" => Some(PassportId::Hgt(data)),
        "hcl" => Some(PassportId::Hcl(data)),
        "ecl" => Some(PassportId::Ecl(data)),
        "pid" => Some(PassportId::Pid(data)),
        "cid" => Some(PassportId::Cid(data)),
        _ => None,
    }
}

fn read_passport_id(raw_input: &str) -> Vec<Option<PassportId>> {
    let re = Regex::new(r"(?P<type>\w{3}).(?P<data>\S+)").unwrap();
    let mut passport_id_list: Vec<Option<PassportId>> = Vec::new();

    for (n, i) in raw_input.lines().enumerate() {
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
enum Height {
    Cm(i32),
    Inch(i32),
}

impl Height {
    fn parse(input: &str) -> Option<Height> {
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
            "cm" => Some(Height::Cm(num)),
            "in" => Some(Height::Inch(num)),
            _ => None,
        }
    }
}

fn is_valid(id: &PassportId) -> bool {
    match id {
        PassportId::Byr(x) => parse_year(x, 1920, 2002),
        PassportId::Iyr(x) => parse_year(x, 2010, 2020),
        PassportId::Eyr(x) => parse_year(x, 2020, 2030),
        PassportId::Hgt(x) => {
            let height = Height::parse(x);
            if height.is_none() {
                return false;
            }
            let height = height.unwrap();
            match height {
                Height::Cm(x) => (150..=193).contains(&x),
                Height::Inch(x) => (59..=76).contains(&x),
            }
        }
        PassportId::Hcl(x) => {
            let re = Regex::new(r"\#(?P<hexs>[0-9a-f]+)").unwrap();
            let cap = re.captures(x);
            if cap.is_none() {
                return false;
            }
            let cap = cap.unwrap();
            let cap = cap["hexs"].to_string();
            cap.len() == 6
        }
        PassportId::Ecl(x) => matches!(
            x.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ),
        PassportId::Pid(x) => {
            let re = Regex::new(r"(?P<id>\d+)").unwrap();
            let cap = re.captures(x);
            let cap = cap.unwrap();
            let cap = cap["id"].to_string();
            cap.len() == 9
        }
        PassportId::Cid(_) => true,
    }
}

fn count_passports(input: &[Option<PassportId>], check_field: bool) -> u32 {
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
                let accept = is_valid(x);
                if !accept && check_field {
                    skip_until_none = true;
                    continue;
                }
                current += 1;
                if let PassportId::Cid(_) = x {
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