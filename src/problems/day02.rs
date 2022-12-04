fn win_score(opponent: &str, mine: &str) -> i32 {
    match mine {
        "X" => match opponent {
            "A" => 3,
            "B" => 0,
            "C" => 6,
            _ => panic!("Invalid shape"),
        },
        "Y" => match opponent {
            "A" => 6,
            "B" => 3,
            "C" => 0,
            _ => panic!("Invalid shape"),
        },
        "Z" => match opponent {
            "A" => 0,
            "B" => 6,
            "C" => 3,
            _ => panic!("Invalid shape"),
        },
        _ => panic!("Invalid shape"),
    }
}

fn shape_from_outcome<'a>(opponent: &str, outcome: &str) -> &'a str {
    match outcome {
        "X" => match opponent {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            _ => panic!("Invalid shape"),
        },
        "Y" => match opponent {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            _ => panic!("Invalid shape"),
        },
        "Z" => match opponent {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            _ => panic!("Invalid shape"),
        },
        _ => panic!("Invalid shape"),
    }
}

pub fn part1(input: &[String]) -> i32 {
    let mut score = 0;

    for line in input {
        let mut iter = line.split_whitespace();
        let opponent = iter.next().unwrap();
        let mine = iter.next().unwrap();

        let shape_score = match mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid shape"),
        };

        let outcome_score = win_score(opponent, mine);

        score += shape_score + outcome_score;
    }

    score
}

pub fn part2(input: &[String]) -> i32 {
    let mut score = 0;

    for line in input {
        let mut iter = line.split_whitespace();
        let opponent = iter.next().unwrap();
        let outcome = iter.next().unwrap();

        let mine = shape_from_outcome(opponent, outcome);

        let shape_score = match mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid shape"),
        };

        let outcome_score = win_score(opponent, mine);

        score += shape_score + outcome_score;
    }

    score
}
