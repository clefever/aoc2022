use problems::day06;

mod advent_of_code;

mod problems {
    automod::dir!(pub "src/problems");
}

fn main() {
    let input = advent_of_code::read_input(6);
    advent_of_code::answer(1, Some(1175), day06::part1(&input));
    advent_of_code::answer(2, Some(3217), day06::part2(&input));
}
