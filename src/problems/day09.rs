use fxhash::FxHashSet;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(9);
    advent_of_code::answer(1, Some(6175), part1(&input));
    advent_of_code::answer(2, Some(2578), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut positions = FxHashSet::default();
    positions.insert((0, 0));

    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;

    for line in input {
        let tokens: Vec<_> = line.split_whitespace().collect();

        let dist: i32 = tokens[1].parse().unwrap();

        for _ in 0..dist {
            match tokens[0] {
                "U" => head_y += 1,
                "D" => head_y -= 1,
                "R" => head_x += 1,
                "L" => head_x -= 1,
                _ => panic!("Not a valid instruction"),
            }

            (tail_x, tail_y) = calculate_move(head_x, head_y, tail_x, tail_y);

            positions.insert((tail_x, tail_y));
        }
    }

    positions.len() as i32
}

fn part2(input: &[String]) -> i32 {
    let mut positions = FxHashSet::default();
    positions.insert((0, 0));

    let mut ropes = vec![(0, 0); 10];

    for line in input {
        let tokens: Vec<_> = line.split_whitespace().collect();

        let dist: i32 = tokens[1].parse().unwrap();

        for _ in 0..dist {
            let (head_x, head_y) = ropes.get_mut(0).unwrap();

            match tokens[0] {
                "U" => *head_y += 1,
                "D" => *head_y -= 1,
                "R" => *head_x += 1,
                "L" => *head_x -= 1,
                _ => panic!("Not a valid instruction"),
            }

            for i in 1..ropes.len() {
                let (x1, y1) = *ropes.get(i - 1).unwrap();
                let (x2, y2) = *ropes.get(i).unwrap();
                let (x, y) = calculate_move(x1, y1, x2, y2);

                let (d_x, d_y) = ropes.get_mut(i).unwrap();

                *d_x = x;
                *d_y = y;

                if i == ropes.len() - 1 {
                    positions.insert((x, y));
                }
            }
        }
    }

    positions.len() as i32
}

fn calculate_move(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32) {
    let mut x2 = x2;
    let mut y2 = y2;

    if x1 - x2 > 1 {
        x2 += 1;
        if y1 - y2 > 0 {
            y2 += 1;
        } else if y2 - y1 > 0 {
            y2 -= 1;
        }
    } else if x2 - x1 > 1 {
        x2 -= 1;
        if y1 - y2 > 0 {
            y2 += 1;
        } else if y2 - y1 > 0 {
            y2 -= 1;
        }
    } else if y1 - y2 > 1 {
        y2 += 1;
        if x1 - x2 > 0 {
            x2 += 1;
        } else if x2 - x1 > 0 {
            x2 -= 1;
        }
    } else if y2 - y1 > 1 {
        y2 -= 1;
        if x1 - x2 > 0 {
            x2 += 1;
        } else if x2 - x1 > 0 {
            x2 -= 1;
        }
    }

    (x2, y2)
}
