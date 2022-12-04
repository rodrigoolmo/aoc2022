use std::vec;

enum Play {
    Rock,
    Paper,
    Scissors
}

impl Play {
    fn score_play(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    fn score_against(&self, other: &Self) -> i32 {
        match (self, other) {
            (Self::Rock, Self::Rock) => 3,
            (Self::Paper, Self::Paper) => 3,
            (Self::Scissors, Self::Scissors) => 3,
            (Self::Rock, Self::Scissors) => 6,
            (Self::Scissors, Self::Paper) => 6,
            (Self::Paper, Self::Rock) => 6,
            (Self::Scissors, Self::Rock) => 0,
            (Self::Rock, Self::Paper) => 0,
            (Self::Paper, Self::Scissors) => 0
        }
    }
}

pub fn part1(input: String) -> String {
    let result: i32 = input.lines().map(|line| {
        let (opp_char, my_char) = line.split_once(" ").unwrap();
        let opp_play = match opp_char {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!()
        };

        let my_play = match my_char {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!()
        };

        my_play.score_play() + my_play.score_against(&opp_play)
    }).sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let result: i32 = input.lines().map(|line| {
        let (opp_char, score_play_char) = line.split_once(" ").unwrap();
        let opp_play = match opp_char {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!()
        };

        let score_against = match score_play_char {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!()
        };

        let possible_plays = vec![Play::Rock, Play::Scissors, Play::Paper];

        let my_play = possible_plays.iter().find(|pos| {
            pos.score_against(&opp_play) == score_against
        }).unwrap();

        my_play.score_play() + score_against
    }).sum();

    result.to_string()
}