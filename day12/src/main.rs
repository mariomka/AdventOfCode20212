use helpers::{input_grid, run, Grid};

extern crate helpers;

fn main() {
    let input: Grid<char> = input_grid(include_str!("../input.txt"));

    run("part1", || day12::part1(&input));
    run("part2", || day12::part2(&input));
}
