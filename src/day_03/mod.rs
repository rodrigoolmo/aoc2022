use std::collections::HashSet;

fn get_priority(letter: char) -> i32 {
    let ascii_value = letter as i32;

    if ascii_value >= 97 && ascii_value <= 122 {
        // a to z
        ascii_value - 96
    } else if ascii_value >= 65 && ascii_value <= 90 {
        // A to Z
        (ascii_value - 64) + 26
    } else {
        panic!()
    }
}

pub fn part1(input: String) -> String {
    let result: i32 = input.lines().map(|line| {
        let num_elements = line.len();
        let (part_1, part_2) = line.split_at(num_elements / 2);

        let elems_1 = part_1.chars().collect::<HashSet<char>>();
        let elems_2 = part_2.chars().collect::<HashSet<char>>();
        let common_elem: char = *elems_1.intersection(&elems_2).next().unwrap();

        get_priority(common_elem)
    }).sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let lines: Vec<_> = input.lines().collect();
    let size_group = 3;
    let num_groups = lines.len() / size_group;

    let result: i32 = (0..num_groups).map(|id_group| {
        let subgroups: Vec<HashSet<char>> = lines[id_group*size_group..id_group*size_group + size_group].iter().map(|&e| HashSet::from_iter(e.chars())).collect();

        let common_elems = subgroups.into_iter().reduce(|set, next| {
            set.intersection(&next).map(|c| *c).collect::<HashSet<char>>()
        }).unwrap();

        let common_elem = common_elems.iter().next().unwrap();

        get_priority(*common_elem)
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day_03::get_priority;

    #[test]
    fn get_priority_values() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }
}