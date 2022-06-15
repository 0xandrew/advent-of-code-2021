use crate::utils::load_file;

pub fn part_a(input: &mut Vec<i32>) -> i32 {
    input.windows(2).filter(|x| x[0] < x[1]).count() as i32
}

pub fn part_b(input: &mut Vec<i32>) -> i32 {
    input
        .windows(3)
        .zip(input.windows(3).skip(1))
        .filter(|(a, b)| a.iter().sum::<i32>() < b.iter().sum::<i32>())
        .count() as i32
}

pub fn main() {
    let mut input = load_file("01")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("---------------------- Sonar Sweep ----------------------");
    println!("part_a: {}", part_a(&mut input));
    println!("part_b: {}", part_b(&mut input));
}
