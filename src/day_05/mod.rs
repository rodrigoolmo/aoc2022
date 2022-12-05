use std::{collections::BTreeMap};

use crate::utils::to_i32;

fn parse_stacks(input: &str) -> BTreeMap<i32, Vec<char>> {
    let mut stacks: BTreeMap<i32, Vec<char>> = BTreeMap::new();

    input.lines().for_each(|line| {
        let chars = Vec::from_iter(line.chars());
        let chunks = chars.chunks(4);

        chunks.enumerate().for_each(|(i, chunk)| {
            if chunk[0] == '[' {
                let element = chunk[1];
                let key = i as i32 + 1;

                let stack_opt = &mut stacks.get_mut(&key);
                match stack_opt {
                    Some(stack) => { stack.push(element); },
                    None => { stacks.insert(key, vec![element]); }
                }
            }
        });
    });

    // Stack order reverse
    stacks.iter_mut().for_each(|(_, stack)| stack.reverse());

    stacks
}

pub fn part1(input: String) -> String {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(part_1);

    // Rearrangement
    part_2.lines().for_each(|line| {
        let step_1 = line.split_once("move ").unwrap().1;
        let (amount_str, step_2) = step_1.split_once(" from ").unwrap();
        let (from_str, to_str) = step_2.split_once(" to ").unwrap();

        let (amount, from, to) = (to_i32(amount_str), to_i32(from_str), to_i32(to_str));

        for _ in 0..amount {
            let elem = stacks.get_mut(&from).unwrap().pop().unwrap();
            stacks.get_mut(&to).unwrap().push(elem);
        }
    });

    let results: Vec<String> = stacks.iter().map(|(_k, v)| v.last().unwrap().to_string()).collect();

    results.join("")
}

pub fn part2(input: String) -> String {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(part_1);

    // Rearrangement
    part_2.lines().for_each(|line| {
        let step_1 = line.split_once("move ").unwrap().1;
        let (amount_str, step_2) = step_1.split_once(" from ").unwrap();
        let (from_str, to_str) = step_2.split_once(" to ").unwrap();

        let (amount, from, to) = (to_i32(amount_str), to_i32(from_str), to_i32(to_str));

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..amount {
            let elem = stacks.get_mut(&from).unwrap().pop().unwrap();
            tmp.push(elem);
        }

        while !tmp.is_empty() {
            let elem = tmp.pop().unwrap();
            stacks.get_mut(&to).unwrap().push(elem);
        }
    });

    let results: Vec<String> = stacks.iter().map(|(_k, v)| v.last().unwrap().to_string()).collect();

    results.join("")
}