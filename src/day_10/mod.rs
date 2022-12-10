use std::{collections::HashSet};

use crate::utils::to_i32;

#[derive(Copy, Clone)]
enum Instruction {
    AddX(i32),
    Noop
}

struct CPU {
    cycle: i32,
    x_register: i32,
    wait_cycles: i32,
    signal_strengths: Vec<i32>,
    pending_instructions: Vec<Instruction>,
    current_instruction: Option<Instruction>,
    pixels_drawn: HashSet<(i32, i32)>,
    finished: bool
}

impl CPU {
    fn from_string(input: String) -> Self {
        let mut instructions: Vec<Instruction> = input.lines().map(|line| {
            if line.starts_with("addx") {
                Instruction::AddX(to_i32(line.split_once("addx ").unwrap().1))
            } else if line.starts_with("noop") {
                Instruction::Noop
            } else {
                panic!()
            }
        }).collect();

        instructions.reverse();

        Self {
            cycle: 1,
            x_register: 1,
            wait_cycles: 0,
            pending_instructions: instructions,
            signal_strengths: Vec::new(),
            current_instruction: None,
            pixels_drawn: HashSet::new(),
            finished: false
        }
    }

    fn increment_cycle(&mut self) {
        self.draw();
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 { // First part
            self.signal_strengths.push(self.cycle * self.x_register);
        }
        
    }

    fn draw(&mut self) {
        if (self.cycle % 40 - self.x_register).abs() <= 1 {
            self.pixels_drawn.insert((self.cycle / 40, self.cycle % 40));
        }
    }

    fn next_step(&mut self) {
        match self.current_instruction {
            Some(Instruction::AddX(num)) => {
                self.wait_cycles -= 1;
                if self.wait_cycles == 0 {
                    self.x_register += num;
                    self.current_instruction = None;
                }
                self.increment_cycle();
            },
            Some(Instruction::Noop) => {
                self.wait_cycles -= 1;
                if self.wait_cycles == 0 {
                    self.current_instruction = None;
                }
                self.increment_cycle();
            },
            None => {
                let instruction = self.pending_instructions.pop();
                self.current_instruction = instruction;
                match instruction {
                    Some(Instruction::AddX(_)) => {
                        self.wait_cycles = 2;
                    },
                    Some(Instruction::Noop) => {
                        self.wait_cycles = 1;
                    },
                    None => {
                        self.finished = true;
                    }
                }
            }
        }
    }

    fn display(&self) -> String {
        let mut output = String::new();
        output.push_str("\n");
        for i in 0..6 {
            for j in 0..40 {
                if self.pixels_drawn.contains(&(i, j)) {
                    output.push_str("#");
                } else {
                    output.push_str(" ")
                }
            }
            output.push_str("\n");
        }
        output
    }
}

pub fn part1(input: String) -> String {
    let mut cpu = CPU::from_string(input);
    while !cpu.finished {
        cpu.next_step();
    }

    let result: i32 = cpu.signal_strengths.iter().sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let mut cpu = CPU::from_string(input);
    while !cpu.finished {
        cpu.next_step();
    }

    let result = cpu.display();

    result
}