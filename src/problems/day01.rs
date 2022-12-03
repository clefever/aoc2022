use std::fs;

pub fn part1() {
    let contents =
        fs::read_to_string("src/input/day01_input.txt").expect("Could not read input file");

    let mut sums = Vec::new();
    let mut curr_sum = 0;

    for num in contents.lines() {
        if num == "" {
            sums.push(curr_sum);
            curr_sum = 0;
        } else {
            let parsed: i32 = num.parse().expect("Not a valid number");
            curr_sum += parsed;
        }
    }

    println!("{}", sums.iter().max().unwrap());
}

pub fn part2() {
    let contents =
        fs::read_to_string("src/input/day01_input.txt").expect("Could not read input file");

    let mut sums = Vec::new();
    let mut curr_sum = 0;

    for num in contents.lines() {
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
    println!("{}", top_three.sum::<i32>());
}
