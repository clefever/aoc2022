use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(1);
    let sums = parse_sums(&input);
    advent_of_code::answer(1, Some(68775), part1(&sums));
    advent_of_code::answer(2, Some(202585), part2(&sums));
}

fn part1(sums: &Vec<i32>) -> i32 {
    *sums.iter().max().unwrap()
}

fn part2(sums: &Vec<i32>) -> i32 {
    let mut sums = sums.to_vec();
    sums.sort();
    let len = sums.len();
    sums.iter().skip(len - 3).sum::<i32>()
}

fn parse_sums(input: &[String]) -> Vec<i32>
{
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

    sums
}
