use itertools::Itertools;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input(6);
    advent_of_code::answer(1, Some(1175), part1(&input));
    advent_of_code::answer(2, Some(3217), part2(&input));
}

fn part1(input: &str) -> i32 {
    for i in 0..=input.len() - 4 {
        let marker = &input[i..i + 4];
        let folded = marker.chars().unique();
        if marker.len() == folded.count() {
            return i as i32 + 4;
        }
    }

    -1
}

fn part2(input: &str) -> i32 {
    for i in 0..=input.len() - 14 {
        let marker = &input[i..i + 14];
        let folded = marker.chars().unique();
        if marker.len() == folded.count() {
            return i as i32 + 14;
        }
    }

    -1
}
