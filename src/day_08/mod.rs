type Grid = Vec<Vec<i32>>;

struct Map {
    grid: Grid,
    max_x: usize,
    max_y: usize
}

fn parse_input(input: String) -> Map {
    let grid: Grid = input.lines()
        .map(|line|
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>())
        .collect::<Vec<_>>();
    Map {
        max_x: grid.len(),
        max_y: grid[0].len(),
        grid
    }
}

fn num_visible_trees(map: Map) -> i32 {
    let mut num = 0;
    for x in 0..map.max_x {
        for y in 0..map.max_y {
            let height = map.grid[x][y];
            if  (0..x).map(|x2| map.grid[x2][y]).all(|height2| height2 < height) ||
                (0..y).map(|y2| map.grid[x][y2]).all(|height2| height2 < height) ||
                (x+1..map.max_x).map(|x2| map.grid[x2][y]).all(|height2| height2 < height) ||
                (y+1..map.max_y).map(|y2| map.grid[x][y2]).all(|height2| height2 < height) {
                num += 1;
            }
        }
    }

    num
}

fn scenic_score(height: i32, it: &mut impl Iterator<Item = i32>) -> i32 {
    match it.next() {
        Some(height2) => {
            match height2 < height {
                true  => 1 + scenic_score(height, it),
                false => 1
            }
        }
        None => 0
    }
}

fn max_scenic_score(map: Map) -> i32 {
    let mut max = 0;
    for x in 0..map.max_x {
        for y in 0..map.max_y {
            let height = map.grid[x][y];
            let value =
                scenic_score(height, &mut (0..x).rev().map(|x2| map.grid[x2][y])) *
                scenic_score(height, &mut (0..y).rev().map(|y2| map.grid[x][y2])) *
                scenic_score(height, &mut (x+1..map.max_x).map(|x2| map.grid[x2][y])) *
                scenic_score(height, &mut (y+1..map.max_y).map(|y2| map.grid[x][y2]));

            max = max.max(value);
        }
    }

    max
}

pub fn part1(input: String) -> String {
    let map: Map = parse_input(input);
    let result = num_visible_trees(map);

    result.to_string()
}

pub fn part2(input: String) -> String {
    let map: Map = parse_input(input);
    let result = max_scenic_score(map);

    result.to_string()
}