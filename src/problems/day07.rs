use fxhash::FxHashMap;
use itertools::Itertools;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(7);
    let sizes = get_directory_sizes(&input);
    advent_of_code::answer(1, Some(1084134), part1(&sizes));
    advent_of_code::answer(2, Some(6183184), part2(&sizes));
}

fn part1(sizes: &FxHashMap<String, i32>) -> i32 {
    sizes.values().filter(|x| **x <= 100_000).sum()
}

fn part2(sizes: &FxHashMap<String, i32>) -> i32 {
    let required = sizes.get("/").unwrap() - 40_000_000;
    *sizes.values().filter(|x| **x >= required).min().unwrap()
}

fn get_directory_sizes(input: &[String]) -> FxHashMap<String, i32>
{
    let mut sizes: FxHashMap<String, i32> = FxHashMap::default();
    let mut dirs = Vec::new();

    for line in input {
        if line.starts_with("$ cd") {
            let tokens: Vec<_> = line.split_whitespace().collect();
            match tokens[2] {
                ".." => {
                    dirs.pop();
                    ()
                }
                x => dirs.push(x),
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let size: i32 = tokens[0].parse().unwrap();

            for i in 1..dirs.len() + 1 {
                let path = dirs.iter().take(i).join("/");

                if sizes.contains_key(&path) {
                    let dir = sizes.get_mut(&path).unwrap();
                    *dir += size;
                } else {
                    sizes.insert(path, size);
                }
            }
        }
    }

    sizes
}
