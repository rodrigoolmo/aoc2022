use std::collections::HashSet;

pub fn part1(input: String) -> String {
    let len_message = 4;
    let mut result: Option<usize> = None;
    let chars: Vec<char> = input.chars().collect();

    let mut idx = 0;
    while result.is_none() {
        let set: HashSet<char> = HashSet::from_iter(chars[idx..idx+len_message].iter().cloned());
        if set.len() == len_message {
            result = Some(idx + len_message);
        }
        idx += 1;
    }

    result.unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let len_message = 14;
    let mut result: Option<usize> = None;
    let chars: Vec<char> = input.chars().collect();

    let mut idx = 0;
    while result.is_none() {
        let set: HashSet<char> = HashSet::from_iter(chars[idx..idx+len_message].iter().cloned());
        if set.len() == len_message {
            result = Some(idx + len_message);
        }
        idx += 1;
    }

    result.unwrap().to_string()
}