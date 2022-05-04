use regex::Regex;
use std::fs;
mod day_01;
mod day_02;
mod day_03;
mod day_04;

#[derive(Debug)]
pub struct Day {
    pub day: u8,
    pub part_one: String,
    pub part_two: String,
}

#[derive(Debug)]
pub enum AocErr {
    InputNotFound,
    InputNotValid,
    DayNotValid,
}

impl Day {
    pub fn new(filename: &str) -> Result<Day, AocErr> {
        let re = Regex::new(r"(?P<day>\d{2})").unwrap();
        let day = re.captures(filename);
        if day.is_none() {
            return Err(AocErr::InputNotFound);
        }
        let day = day
            .unwrap()
            .name("day")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        let input = fs::read_to_string(filename);
        if input.is_err() {
            return Err(AocErr::InputNotFound);
        }
        let input = input.unwrap();

        match day {
            1 => Ok(day_01::parse(&input)),
            2 => Ok(day_02::parse(&input)),
            3 => Ok(day_03::parse(&input)),
            4 => Ok(day_04::parse(&input)),
            _ => Err(AocErr::DayNotValid),
        }
    }
}
