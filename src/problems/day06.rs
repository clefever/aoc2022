use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    for i in 0..=input.len() - 4 {
        let marker = &input[i..i + 4];
        let folded = marker.chars().unique();
        if marker.len() == folded.count() {
            return i as i32 + 4;
        }
    }

    -1
}

pub fn part2(input: &str) -> i32 {
    for i in 0..=input.len() - 14 {
        let marker = &input[i..i + 14];
        let folded = marker.chars().unique();
        if marker.len() == folded.count() {
            return i as i32 + 14;
        }
    }

    -1
}
