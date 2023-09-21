use itertools::Itertools;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input(6);
    advent_of_code::answer(1, Some(1175), part1(&input));
    advent_of_code::answer(2, Some(3217), part2(&input));
}

fn part1(input: &str) -> i32 {
    find_start_of_packet(input, 4)
}

fn part2(input: &str) -> i32 {
    find_start_of_packet(input, 14)
}

fn find_start_of_packet(input: &str, num_unique: usize) -> i32 {
    for i in 0..=input.len() - num_unique {
        let marker = &input[i..i + num_unique];
        if marker.len() == marker.chars().unique().count() {
            return (i + num_unique) as i32;
        }
    }

    -1
}
