use std::io;
use std::fs;

mod day_01;

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
        _ => { "Invalid input".to_string() }
    };

    println!("Result: {}", result);
}
