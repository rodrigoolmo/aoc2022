use std::io;
use std::fs;

mod utils;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

pub enum TypeInput {
    Small,
    Large
}

impl TypeInput {
    fn from_str(input: &str) -> Self {
        match input {
            "sm" => Self::Small,
            "lg" => Self::Large,
            _ => panic!()
        }
    }

    fn file_name(&self, day: &str) -> String {
        match self {
            Self::Small => format!("inputs/{}/input_sm.txt", day),
            Self::Large => format!("inputs/{}/input.txt", day)
        }
    }

    fn get_input(&self, day: &str) -> String {
        fs::read_to_string(self.file_name(day)).unwrap()
    }
}

fn main() {
    println!("Select day (1 to 25), part (1 or 2) and type of input (sm or lg).");
    println!("Example: 1.1 sm");
    println!("=> ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let (day, type_input) = input.trim().split_once(" ").expect("Malformed input");

    let type_input = TypeInput::from_str(type_input);

    let result = match day.trim() {
        "1.1" => { day_01::part1(type_input.get_input("01")) }
        "1.2" => { day_01::part2(type_input.get_input("01")) }
        "2.1" => { day_02::part1(type_input.get_input("02")) }
        "2.2" => { day_02::part2(type_input.get_input("02")) }
        "3.1" => { day_03::part1(type_input.get_input("03")) }
        "3.2" => { day_03::part2(type_input.get_input("03")) }
        "4.1" => { day_04::part1(type_input.get_input("04")) }
        "4.2" => { day_04::part2(type_input.get_input("04")) }
        "5.1" => { day_05::part1(type_input.get_input("05")) }
        "5.2" => { day_05::part2(type_input.get_input("05")) }
        "6.1" => { day_06::part1(type_input.get_input("06")) }
        "6.2" => { day_06::part2(type_input.get_input("06")) }
        "7.1" => { day_07::part1(type_input.get_input("07")) }
        "7.2" => { day_07::part2(type_input.get_input("07")) }
        _ => { "Invalid input".to_string() }
    };

    println!("Result: {}", result);
}
