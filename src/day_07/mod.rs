use std::{collections::HashMap};

use crate::utils::to_i32;

#[derive(Debug)]
struct Dir {
    father: String,
    files: Vec<(String, i32)>,
    dirs: Vec<String>,
    size: Option<i32>
}

fn parse_input(input: String) -> HashMap<String, Dir> {
    let mut filesistem: HashMap<String, Dir> = HashMap::new();

    let mut curr_dir_abs = "/".to_string();
    filesistem.insert("/".to_string(), Dir {
        father: "".to_string(),
        files: Vec::new(),
        dirs: Vec::new(),
        size: None
    });

    for line in input.lines().skip(1) {
        let father = filesistem.get(&curr_dir_abs).unwrap().father.clone();
        if line.starts_with("$ cd") {
            // Change directory
            let (_, new_dir_key_rel) = line.split_once("$ cd ").unwrap();
            match new_dir_key_rel {
                ".." => {
                    curr_dir_abs = father;
                },
                _ => {
                    let new_dir_key_abs = curr_dir_abs + "/" + new_dir_key_rel;
                    curr_dir_abs = new_dir_key_abs;
                }
            }
        } else if line.starts_with("$ ls") {
            // ls, nothing to do
        } else if line.starts_with("dir") {
            // Folders
            let (_, new_dir_key_rel) = line.split_once("dir ").unwrap();
            let new_dir_key_abs = curr_dir_abs.clone() + "/" + new_dir_key_rel;
            let curr_dir = filesistem.get_mut(&curr_dir_abs).unwrap();
            curr_dir.dirs.push(new_dir_key_abs.clone());
            filesistem.insert(new_dir_key_abs, Dir {
                father: curr_dir_abs.clone(),
                files: Vec::new(),
                dirs: Vec::new(),
                size: None
            });
        } else {
            // Files
            let (size_str, file_name) = line.split_once(" ").unwrap();
            let size = to_i32(size_str);
            let curr_dir = filesistem.get_mut(&curr_dir_abs).unwrap();
            curr_dir.files.push((file_name.to_string(), size));
        }
    }
    filesistem
}

fn calculate_size(filesistem: &mut HashMap<String, Dir>, key: &String) -> i32 {
    let dir = filesistem.get(key).unwrap().clone();
    let size_files: i32 = dir.files.iter()
        .map(|(_f, size)| size)
        .sum();
    let mut size_folders = 0;
    for folder in &dir.dirs.clone() {
        size_folders += calculate_size(filesistem, folder);
    } 
    let total_size = size_files + size_folders;
    filesistem.entry(key.clone()).and_modify(|e| e.size = Some(total_size));
    total_size
}

pub fn part1(input: String) -> String {
    let mut filesistem = parse_input(input);
    calculate_size(&mut filesistem, &"/".to_string());

    let result: i32 = filesistem.iter()
        .map(|(_k, dir)| dir.size.unwrap())
        .filter(|&size| size <= 100_000).sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let mut filesistem = parse_input(input);
    calculate_size(&mut filesistem, &"/".to_string());

    let size_total = filesistem.get(&"/".to_string()).unwrap().size.unwrap();
    let space_to_free = 30_000_000 - (70_000_000 - size_total);

    let result = filesistem.iter()
        .map(|(_k, v)|v.size.unwrap())
        .filter(|&size| size > space_to_free).min().unwrap();

    result.to_string()
}