use itertools::Itertools;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(10);
    advent_of_code::answer(1, Some(12980), part1(&input));
    advent_of_code::answer(2, None, part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut milestones = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    let mut cycles = 0;
    let mut reg_x = 1;

    for line in input {
        if milestones.is_empty() {
            break;
        }

        let cmd: Vec<_> = line.split_whitespace().collect();

        let amt = cmd.get(1).unwrap_or(&"0");

        match cmd[0] {
            "noop" => cycles += 1,
            "addx" => {
                reg_x += amt.parse::<i32>().unwrap();
                cycles += 2;
            }
            _ => panic!("Not a valid command"),
        }

        if cycles >= milestones[0] {
            sum += (reg_x - amt.parse::<i32>().unwrap()) * milestones[0];
            milestones.drain(..1);
        }
    }

    sum
}

fn part2(input: &[String]) -> String {
    let mut pixels = vec![vec!['.'; 40]; 6];

    const MAX_CYCLES: i32 = 240;
    const CYCLES_PER_ROW: i32 = 40;

    let mut reg_x = 1;
    let mut counter;
    let mut cycle = 0;

    let mut iter = input.iter();

    while cycle < MAX_CYCLES {
        let parsed = iter
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        counter = match parsed[0] {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Not a valid command"),
        };

        while counter > 0 {
            let col = cycle % CYCLES_PER_ROW;
            if col >= reg_x - 1 && col <= reg_x + 1 {
                let row = cycle / CYCLES_PER_ROW;
                pixels[row as usize][col as usize] = '#';
            }

            counter -= 1;
            cycle += 1;
        }

        if parsed[0] == "addx" {
            reg_x += parsed[1].parse::<i32>().unwrap();
        }
    }

    let mut answer = String::from("\n");

    answer.push_str(
        &pixels
            .into_iter()
            .map(|vec| vec.iter().collect::<String>())
            .join("\n"),
    );

    answer
}
