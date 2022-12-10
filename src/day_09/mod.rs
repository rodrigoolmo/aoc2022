use std::{collections::{HashSet, VecDeque}, hash::Hash};

use crate::utils::to_i32;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn apply_movement(&self, movement: &Movement) -> Point {
        match movement {
            Movement::Up    => Point { x: self.x + 1, y: self.y },
            Movement::Down  => Point { x: self.x - 1, y: self.y },
            Movement::Right => Point { x: self.x, y: self.y + 1 },
            Movement::Left  => Point { x: self.x, y: self.y - 1 }
        }
    }

    fn apply_point_movement(self: Point, to: Point) -> Point {
        if self == to {
            self
        } else if self.x == to.x && (self.y - to.y).abs() == 2 {
            Point {x: self.x, y: (self.y + to.y) / 2}
        } else if self.y == to.y && (self.x - to.x).abs() == 2 {
            Point {x: (self.x + to.x) / 2, y: self.y}
        } else if (self.x - to.x).abs() == 1 && (self.y - to.y).abs() == 2 {
            Point {x: to.x, y: (self.y + to.y) / 2}
        } else if (self.y - to.y).abs() == 1 && (self.x - to.x).abs() == 2 {
            Point {x: (self.x + to.x) / 2, y: to.y}
        } else if (self.y - to.y).abs() == 2 && (self.x - to.x).abs() == 2 {
            Point {x: (self.x + to.x) / 2, y: (self.y + to.y) / 2}
        } else {
            self
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

impl Movement {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!()
        }
    }
}

struct State {
    tail_positions: HashSet<Point>,
    tail: Point,
    head: Point,
    pending_movements: VecDeque<Movement>
}

impl State {
    fn from_string(s: String) -> Self {
        let mut movements: VecDeque<Movement> = VecDeque::new();
        for line in s.lines() {
            let (direction, amount) = line.split_once(" ")
                .map(|(direction, amount_str)| (direction, to_i32(amount_str))).unwrap();
            (0..amount).for_each(|_| movements.push_back(Movement::from_str(direction)));
        }
    
        Self {
            tail_positions: HashSet::new(),
            tail: Point {x: 0, y: 0},
            head: Point {x: 0, y: 0},
            pending_movements: movements
        }
    }

    fn compute_movement(&mut self, movement: Movement) {
        match &movement {
            Movement::Up => {
                if self.tail.x < self.head.x {
                    self.tail = self.head;
                }
            },
            Movement::Down => {
                if self.tail.x > self.head.x {
                    self.tail = self.head;
                }
            },
            Movement::Right => {
                if self.tail.y < self.head.y {
                    self.tail = self.head;
                }
            },
            Movement::Left => {
                if self.tail.y > self.head.y {
                    self.tail = self.head;
                }
            }
        }
        self.tail_positions.insert(self.tail);
        self.head = self.head.apply_movement(&movement);
    }

    fn compute_movements(&mut self) {
        while let Some(movement) = self.pending_movements.pop_front() {
            self.compute_movement(movement);
        }
    }
}

struct StatePart2 {
    tail_positions: HashSet<Point>,
    knots: [Point; 10],
    pending_movements: VecDeque<Movement>
}

impl StatePart2 {
    fn from_string(s: String) -> Self {
        let mut movements: VecDeque<Movement> = VecDeque::new();
        for line in s.lines() {
            let (direction, amount) = line.split_once(" ")
                .map(|(direction, amount_str)| (direction, to_i32(amount_str))).unwrap();
            (0..amount).for_each(|_| movements.push_back(Movement::from_str(direction)));
        }
    
        Self {
            tail_positions: HashSet::new(),
            knots: [Point{x: 0, y: 0}; 10],
            pending_movements: movements
        }
    }

    fn compute_movement(&mut self, initial_movement: Movement) {
        let mut new_knots: [Point; 10] = self.knots;
        new_knots[0] = self.knots[0].apply_movement(&initial_movement);
        for i in 1..10 {
            new_knots[i] = self.knots[i].apply_point_movement(new_knots[i-1]);
        }
        self.tail_positions.insert(new_knots[9]);
        self.knots = new_knots;
    }

    fn compute_movements(&mut self) {
        while let Some(movement) = self.pending_movements.pop_front() {
            self.compute_movement(movement);
        }
    }
}


pub fn part1(input: String) -> String {
    let mut state = State::from_string(input);
    state.compute_movements();
    let result = state.tail_positions.len();

    result.to_string()
}


pub fn part2(input: String) -> String {
    let mut state = StatePart2::from_string(input);
    state.compute_movements();
    let result = state.tail_positions.len();

    result.to_string()
}