use std::collections::HashSet;

use crate::Day;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Vertex {
    x: u32,
    y: u32,
}

pub fn parse(raw_input: &str) -> Day {
    let (limit, input) = load(raw_input);
    let part_one = travel(3, 1, &limit, &input);

    let mut part_two = travel(1, 1, &limit, &input);
    part_two *= travel(3, 1, &limit, &input);
    part_two *= travel(5, 1, &limit, &input);
    part_two *= travel(7, 1, &limit, &input);
    part_two *= travel(1, 2, &limit, &input);

    Day {
        day: 3,
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    }
}

fn load(raw_input: &str) -> (Vertex, HashSet<Vertex>) {
    let mut cordinate: HashSet<Vertex> = HashSet::new();
    let mut x_length: Option<u8> = None;
    let mut y_length = 0;
    for (y, straight) in raw_input.lines().enumerate() {
        if x_length.is_none() {
            let i = straight.len() as u8;
            x_length = Some(i);
        }
        for (x, target) in straight.chars().enumerate() {
            if target != '#' {
                continue;
            }
            let i = Vertex {
                x: x as u32,
                y: y as u32,
            };
            cordinate.insert(i);
        }
        y_length += 1;
    }
    let length = Vertex {
        x: x_length.unwrap() as u32,
        y: y_length,
    };
    (length, cordinate)
}

fn travel(right: u32, down: u32, limit: &Vertex, input: &HashSet<Vertex>) -> u64 {
    let mut count = 0;
    for i in 1..limit.y {
        let x = i * right % limit.x;
        let y = i * down;
        if y > limit.y {
            break;
        }
        let cord = Vertex { x, y };
        if input.contains(&cord) {
            count += 1;
        }
    }
    count
}
