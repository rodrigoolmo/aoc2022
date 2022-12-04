pub fn part1(input: String) -> String {
    let result: i32 = input.split("\n\n").map(
        |per_elf|
        per_elf.lines().map(|n| n.parse::<i32>().unwrap()).sum()
    ).max().unwrap();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let mut results: Vec<i32> = input.split("\n\n").map(
        |per_elf|
        per_elf.lines().map(|n| n.parse::<i32>().unwrap()).sum()
    ).collect();

    results.sort();

    results.reverse();

    let result: i32 = results.iter().take(3).sum();

    result.to_string()
}