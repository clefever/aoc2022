use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(3);
    advent_of_code::answer(1, Some(7903), part1(&input));
    advent_of_code::answer(2, Some(2548), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;

    for line in input {
        let first_compartment = &line[0..line.len() / 2];
        let second_compartment = &line[line.len() / 2..line.len()];

        'outer: for item in first_compartment.chars() {
            for other in second_compartment.chars() {
                if item == other {
                    sum += get_priority(item);
                    break 'outer;
                }
            }
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut sum = 0;

    let mut iter = input.iter();

    loop {
        let first = match iter.next() {
            Some(x) => x,
            None => break,
        };
        let second = match iter.next() {
            Some(x) => x,
            None => break,
        };
        let third = match iter.next() {
            Some(x) => x,
            None => break,
        };

        'outer: for item in first.chars() {
            for other in second.chars() {
                if item == other {
                    for last in third.chars() {
                        if other == last {
                            sum += get_priority(item);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    sum
}

fn get_priority(item: char) -> i32 {
    let ascii_code = item as u8;
    match ascii_code {
        65..=90 => ascii_code as i32 - 38,
        97..=122 => ascii_code as i32 - 96,
        _ => panic!("Not valid item code"),
    }
}
