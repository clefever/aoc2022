use problems::day05;

mod advent_of_code;

mod problems {
    automod::dir!(pub "src/problems");
}

fn main() {
    let input = advent_of_code::read_input(5);
    advent_of_code::answer(1, Some(String::from("FZCMJCRHZ")), day05::part1(&input));
    advent_of_code::answer(2, Some(String::from("JSDHQMZGF")), day05::part2(&input));
}
