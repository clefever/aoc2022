use std::fs;

fn get_priority(item: char) -> i32 {
    let ascii_code = item as u8;
    match ascii_code {
        65..=90 => ascii_code as i32 - 38,
        97..=122 => ascii_code as i32 - 96,
        _ => panic!("Not valid item code"),
    }
}

pub fn part1() {
    let contents =
        fs::read_to_string("src/input/day03_input.txt").expect("Could not read input file");

    let mut sum = 0;

    for line in contents.lines() {
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

    println!("{}", sum);
}

pub fn part2() {
    let contents =
        fs::read_to_string("src/input/day03_input.txt").expect("Could not read input file");

    let mut sum = 0;

    let mut iter = contents.lines();

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

    println!("{}", sum);
}
