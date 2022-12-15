use helpers::{input_lines, run};

fn main() {
    let input: Vec<&str> = input_lines(include_str!("../input.txt"));

    run("part1", || day15::part1(&input, 2000000));
    run("part2", || day15::part2(&input, 0, 4000000));
}
