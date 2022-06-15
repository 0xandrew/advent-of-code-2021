mod programs;
mod utils;

fn run(day: i32) {
    match day {
        1 => programs::day_01::main(),
        2 => programs::day_02::main(),
        _ => println!("I don't have solution to day {}", day),
    }
}

fn main() {
    run(2);
}
