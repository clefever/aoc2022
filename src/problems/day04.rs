use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(4);
    advent_of_code::answer(1, Some(550), part1(&input));
    advent_of_code::answer(2, Some(931), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;

    for line in input {
        let jobs: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = jobs[0].split('-').collect();
        let second: Vec<&str> = jobs[1].split('-').collect();
        let first_begin: i32 = first[0].parse().unwrap();
        let first_end: i32 = first[1].parse().unwrap();
        let second_begin: i32 = second[0].parse().unwrap();
        let second_end: i32 = second[1].parse().unwrap();

        if (second_begin >= first_begin && second_end <= first_end)
            || (first_begin >= second_begin && first_end <= second_end)
        {
            sum += 1;
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut sum = 0;

    for line in input {
        let jobs: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = jobs[0].split('-').collect();
        let second: Vec<&str> = jobs[1].split('-').collect();
        let first_begin: i32 = first[0].parse().unwrap();
        let first_end: i32 = first[1].parse().unwrap();
        let second_begin: i32 = second[0].parse().unwrap();
        let second_end: i32 = second[1].parse().unwrap();

        'outer: for f in first_begin..=first_end {
            for s in second_begin..=second_end {
                if f == s {
                    sum += 1;
                    break 'outer;
                }
            }
        }
    }

    sum
}
