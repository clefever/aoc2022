use fxhash::FxHashMap;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(5);
    advent_of_code::answer(1, Some(String::from("FZCMJCRHZ")), part1(&input));
    advent_of_code::answer(2, Some(String::from("JSDHQMZGF")), part2(&input));
}

fn part1(input: &[String]) -> String {
    let mut stacks: FxHashMap<i32, Vec<String>> = FxHashMap::default();

    let columns = (input[0].len() as f64 / 4.0).ceil() as i32;

    for i in 0..columns {
        stacks.insert(i + 1, Vec::new());
    }

    'outer: for line in input {
        let mut column = 1;
        let chars = line.chars().collect::<Vec<char>>();
        let mut i: usize = 0;
        while i < chars.len() {
            let ch = chars.get(i);
            match ch {
                Some('[') => {
                    let num = chars.get(i + 1).unwrap().to_string();
                    let vec = stacks.get_mut(&column).unwrap();
                    vec.push(num);
                    i += 3;
                }
                Some(' ') => {
                    i += 3;
                }
                Some('1') => {
                    break 'outer;
                }
                _ => break,
            }

            column += 1;
            i += 1;
        }
    }

    for line in input {
        if line.starts_with("move") {
            let moves: Vec<&str> = line.split_whitespace().collect();
            let amount: usize = moves[1].parse().unwrap();
            let source: i32 = moves[3].parse().unwrap();
            let dest: i32 = moves[5].parse().unwrap();

            let to_move: Vec<_> = {
                let source_vec = stacks.get_mut(&source).unwrap();
                source_vec.drain(0..amount).collect()
            };

            let dest_vec = stacks.get_mut(&dest).unwrap();

            for ch in to_move.iter() {
                dest_vec.insert(0, ch.to_string());
            }
        }
    }

    let mut sum = Vec::new();

    for col in 1..columns + 1 {
        let ch = match stacks.get(&col).unwrap().get(0) {
            Some(x) => x.to_string(),
            _ => String::from(""),
        };
        sum.push(ch);
    }

    sum.join("")
}

fn part2(input: &[String]) -> String {
    let mut stacks: FxHashMap<i32, Vec<String>> = FxHashMap::default();

    let columns = (input[0].len() as f64 / 4.0).ceil() as i32;

    for i in 0..columns {
        stacks.insert(i + 1, Vec::new());
    }

    'outer: for line in input {
        let mut column = 1;
        let chars = line.chars().collect::<Vec<char>>();
        let mut i: usize = 0;
        while i < chars.len() {
            let ch = chars.get(i);
            match ch {
                Some('[') => {
                    let num = chars.get(i + 1).unwrap().to_string();
                    let vec = stacks.get_mut(&column).unwrap();
                    vec.push(num);
                    i += 3;
                }
                Some(' ') => {
                    i += 3;
                }
                Some('1') => {
                    break 'outer;
                }
                _ => break,
            }

            column += 1;
            i += 1;
        }
    }

    for line in input {
        if line.starts_with("move") {
            let moves: Vec<&str> = line.split_whitespace().collect();
            let amount: usize = moves[1].parse().unwrap();
            let source: i32 = moves[3].parse().unwrap();
            let dest: i32 = moves[5].parse().unwrap();

            let to_move: Vec<_> = {
                let source_vec = stacks.get_mut(&source).unwrap();
                source_vec.drain(0..amount).collect()
            };

            let dest_vec = stacks.get_mut(&dest).unwrap();

            for ch in to_move.iter().rev() {
                dest_vec.insert(0, ch.to_string());
            }
        }
    }

    let mut sum = Vec::new();

    for col in 1..columns + 1 {
        let ch = match stacks.get(&col).unwrap().get(0) {
            Some(x) => x.to_string(),
            _ => String::from(""),
        };
        sum.push(ch);
    }

    sum.join("")
}
