use problems::day03;

mod advent_of_code;

mod problems {
    automod::dir!(pub "src/problems");
}

fn main() {
    let input = advent_of_code::read_input(3);
    advent_of_code::answer(1, Some(7903), day03::part1(&input[..]));
    advent_of_code::answer(2, Some(2548), day03::part2(&input[..]));
}
