use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(1);
    advent_of_code::answer(1, Some(68775), part1(&input));
    advent_of_code::answer(2, Some(202585), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut sums = Vec::new();
    let mut curr_sum = 0;

    for num in input {
        if num == "" {
            sums.push(curr_sum);
            curr_sum = 0;
        } else {
            let parsed: i32 = num.parse().expect("Not a valid number");
            curr_sum += parsed;
        }
    }

    *sums.iter().max().unwrap()
}

fn part2(input: &[String]) -> i32 {
    let mut sums = Vec::new();
    let mut curr_sum = 0;

    for num in input {
        if num == "" {
            sums.push(curr_sum);
            curr_sum = 0;
        } else {
            let parsed: i32 = num.parse().expect("Not a valid number");
            curr_sum += parsed;
        }
    }

    sums.sort();
    let len = sums.len();
    let top_three = sums.iter().skip(len - 3);
    top_three.sum::<i32>()
}
