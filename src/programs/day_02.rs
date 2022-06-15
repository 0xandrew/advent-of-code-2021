use crate::utils::load_file;

pub fn part_a(input: &mut str) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for inst in input.lines() {
        let inst_vec = inst.split(' ').collect::<Vec<&str>>();
        let inst_value = inst_vec[1].parse::<i32>().unwrap();

        match inst_vec[0] {
            "forward" => horizontal_position += inst_value,
            "down" => depth += inst_value,
            "up" => depth -= inst_value,
            _ => {}
        }
    }

    horizontal_position * depth
}

pub fn part_b(input: &mut str) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for inst in input.lines() {
        let inst_vec = inst.split(' ').collect::<Vec<&str>>();
        let inst_value = inst_vec[1].parse::<i32>().unwrap();

        match inst_vec[0] {
            "forward" => {
                horizontal_position += inst_value;
                depth += aim * inst_value
            }
            "down" => aim += inst_value,
            "up" => aim -= inst_value,
            _ => {}
        }
    }

    horizontal_position * depth
}
pub fn main() {
    let mut input = load_file("02");
    println!("---------------------- Dive! ----------------------");
    println!("part_a: {}", part_a(&mut input));
    println!("part_b: {}", part_b(&mut input));
}
