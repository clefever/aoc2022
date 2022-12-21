use fxhash::FxHashMap;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(8);
    advent_of_code::answer(1, Some(1708), part1(&input));
    advent_of_code::answer(2, Some(504000), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut trees: FxHashMap<(i32, i32), i32> = FxHashMap::default();

    let mut row = 0;
    let mut col = 0;

    for line in input {
        col = 0;
        for tree in line.chars() {
            trees.insert((row, col), tree.to_digit(10).unwrap() as i32);
            col += 1;
        }
        row += 1;
    }

    let mut sum = (2 * row) + (2 * col) - 4;

    for y in 0..row {
        for x in 0..col {
            if x != 0 && y != 0 && x != col - 1 && y != row - 1 {
                let height = trees.get(&(x, y)).unwrap();

                // Left
                let mut tallest_left = true;
                for i in 0..x {
                    let compare = trees.get(&(i, y)).unwrap();
                    if compare >= height {
                        tallest_left = false;
                    }
                }

                // Right
                let mut tallest_right = true;
                for i in x + 1..row {
                    let compare = trees.get(&(i, y)).unwrap();
                    if compare >= height {
                        tallest_right = false;
                    }
                }

                // Top
                let mut tallest_top = true;
                for i in 0..y {
                    let compare = trees.get(&(x, i)).unwrap();
                    if compare >= height {
                        tallest_top = false;
                    }
                }

                // Bottom
                let mut tallest_bottom = true;
                for i in y + 1..col {
                    let compare = trees.get(&(x, i)).unwrap();
                    if compare >= height {
                        tallest_bottom = false;
                    }
                }

                if tallest_left || tallest_right || tallest_top || tallest_bottom {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut trees: FxHashMap<(i32, i32), i32> = FxHashMap::default();

    let mut row = 0;
    let mut col = 0;

    for line in input {
        col = 0;
        for tree in line.chars() {
            trees.insert((row, col), tree.to_digit(10).unwrap() as i32);
            col += 1;
        }
        row += 1;
    }

    let mut scores = Vec::new();

    for y in 0..row {
        for x in 0..col {
            if x != 0 && y != 0 && x != col - 1 && y != row - 1 {
                let height = trees.get(&(x, y)).unwrap();

                // Left
                let mut dist_left = 0;
                for i in (0..x).rev() {
                    dist_left += 1;
                    let compare = trees.get(&(i, y)).unwrap();
                    if compare >= height {
                        break;
                    }
                }

                // Right
                let mut dist_right = 0;
                for i in x + 1..row {
                    dist_right += 1;
                    let compare = trees.get(&(i, y)).unwrap();
                    if compare >= height {
                        break;
                    }
                }

                // Top
                let mut dist_top = 0;
                for i in (0..y).rev() {
                    dist_top += 1;
                    let compare = trees.get(&(x, i)).unwrap();
                    if compare >= height {
                        break;
                    }
                }

                // Bottom
                let mut dist_bottom = 0;
                for i in y + 1..col {
                    dist_bottom += 1;
                    let compare = trees.get(&(x, i)).unwrap();
                    if compare >= height {
                        break;
                    }
                }

                scores.push(dist_left * dist_right * dist_top * dist_bottom);
            }
        }
    }

    *scores.iter().max().unwrap()
}
