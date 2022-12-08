use helpers::{input_grid, run, Grid};

fn main() {
    let input: Grid<usize> = input_grid(include_str!("../input.txt"));

    run("part1", || day8::part1(&input));
    run("part2", || day8::part2(&input));
}
