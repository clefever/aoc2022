use problems::day04;

mod advent_of_code;

mod problems {
    automod::dir!(pub "src/problems");
}

fn main() {
    let input = advent_of_code::read_input(4);
    advent_of_code::answer(1, Some(550), day04::part1(&input[..]));
    advent_of_code::answer(2, Some(931), day04::part2(&input[..]));
}
