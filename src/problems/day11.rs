use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input(11);
    advent_of_code::answer(1, Some(50830), part1(&input));
    advent_of_code::answer(2, Some(14399640002), part2(&input));
}

struct Monkey {
    inspections: i32,
    items: Vec<i64>,
    second: Option<i32>,
    operator: char,
    test: i32,
    true_path: i32,
    false_path: i32,
}

fn part1(input: &String) -> i64 {
    calc_monkey_business(input, 20, true)
}

fn part2(input: &String) -> i64 {
    calc_monkey_business(input, 10000, false)
}

fn calc_monkey_business(input: &String, rounds: i32, manage_worry: bool) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let monkey_strings: Vec<_> = input.split("\r\n\r\n").collect();

    for monkey in monkey_strings {
        let lines: Vec<_> = monkey.lines().collect();

        let items: Vec<_> = lines[1].split_whitespace().collect();
        let items: Vec<i64> = items[2..]
            .iter()
            .map(|i| i.trim_end_matches(',').parse::<i64>().unwrap())
            .collect();

        let operation: Vec<_> = lines[2].split_whitespace().collect();
        let second = operation[5].parse::<i32>().ok();
        let operator = operation[4].chars().nth(0).unwrap();

        let test = lines[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let true_path: Vec<_> = lines[4].split_whitespace().collect();
        let true_path = true_path.last().unwrap().parse::<i32>().unwrap();

        let false_path: Vec<_> = lines[5].split_whitespace().collect();
        let false_path = false_path.last().unwrap().parse::<i32>().unwrap();

        let monkey = Monkey {
            inspections: 0,
            items,
            second,
            operator,
            test,
            true_path,
            false_path,
        };

        monkeys.push(monkey);
    }

    let modulus: i64 = monkeys.iter().map(|m| m.test as i64).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey3 = monkeys.get(i).unwrap();

            for _ in 0..monkey3.items.len() {
                let values = {
                    let monkey = monkeys.get_mut(i).unwrap();
                    let mut item = monkey.items.drain(..1).collect::<Vec<_>>()[0];
                    monkey.inspections += 1;

                    item = match monkey.operator {
                        '+' => match monkey.second {
                            Some(x) => item + x as i64,
                            None => item + item,
                        },
                        '*' => match monkey.second {
                            Some(x) => item * x as i64,
                            None => item * item,
                        },
                        _ => panic!("Not a valid operator"),
                    } % modulus;

                    if manage_worry {
                        item /= 3;
                    }

                    (item, monkey.test, monkey.true_path, monkey.false_path)
                };

                if values.0 % values.1 as i64 == 0 {
                    let monkey2 = monkeys.get_mut(values.2 as usize).unwrap();
                    monkey2.items.push(values.0);
                } else {
                    let monkey2 = monkeys.get_mut(values.3 as usize).unwrap();
                    monkey2.items.push(values.0);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    monkeys.pop().unwrap().inspections as i64 * monkeys.pop().unwrap().inspections as i64
}
