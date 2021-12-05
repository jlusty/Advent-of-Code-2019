use std::error::Error;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut total_fuel = 0;
    for line in input.lines() {
        let mass: u32 = line.parse().unwrap();
        let fuel = (mass / 3) - 2;
        total_fuel += fuel;
    }

    println!("{}", total_fuel);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut total_fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().unwrap();
        let mut fuel = (mass / 3) - 2;
        let mut extra_fuel = fuel;
        loop {
            extra_fuel = (extra_fuel / 3) - 2;
            if extra_fuel <= 0 {
                break;
            }
            fuel += extra_fuel;
        }
        total_fuel += fuel;
    }

    println!("{}", total_fuel);
}
