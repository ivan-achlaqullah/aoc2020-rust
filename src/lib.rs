use std::fs;
mod day_01;
mod day_02;
mod day_03;

#[derive(Debug)]
pub struct Day {
    pub day: u8,
    pub part_one: String,
    pub part_two: String,
}

#[derive(Debug)]
pub enum AocErr {
    InputNotFound,
    DayNotValid,
}

impl Day {
    pub fn new(day: u8, filename: &str) -> Result<Day, AocErr> {
        let input = fs::read_to_string(filename);
        if input.is_err() {
            return Err(AocErr::InputNotFound);
        }
        let input = input.unwrap();

        match day {
            1 => Ok(day_01::parse(&input)),
            2 => Ok(day_02::parse(&input)),
            3 => Ok(day_03::parse(&input)),
            _ => Err(AocErr::DayNotValid),
        }
    }
}
