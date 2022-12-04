fn to_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

pub fn part1(input: String) -> String {
    let result = input.lines().filter(|line| {
        let (elf_1, elf_2) = line.split_once(",").unwrap();
        let (elf_1_from, elf_1_to) = elf_1.split_once("-").map(|(a,b)| (to_i32(a), to_i32(b))).unwrap();
        let (elf_2_from, elf_2_to) = elf_2.split_once("-").map(|(a,b)| (to_i32(a), to_i32(b))).unwrap();

        if  (elf_1_from >= elf_2_from && elf_1_to <= elf_2_to) ||
            (elf_2_from >= elf_1_from && elf_2_to <= elf_1_to) {
            true
        } else {
            false
        }
    }).count();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let result = input.lines().filter(|line| {
        let (elf_1, elf_2) = line.split_once(",").unwrap();
        let (elf_1_from, elf_1_to) = elf_1.split_once("-").map(|(a,b)| (to_i32(a), to_i32(b))).unwrap();
        let (elf_2_from, elf_2_to) = elf_2.split_once("-").map(|(a,b)| (to_i32(a), to_i32(b))).unwrap();

        if  (elf_1_from >= elf_2_from && elf_1_from <= elf_2_to) ||
            (elf_2_from >= elf_1_from && elf_2_from <= elf_1_to) {
            true
        } else {
            false
        }
    }).count();

    result.to_string()
}