use helpers::{run, split_input};

fn main() {
    let input: Vec<&str> = split_input(include_str!("../input.txt"), "\n\n");

    run("part1", || day1::part1(&input));
    run("part2", || day1::part2(&input));
}
