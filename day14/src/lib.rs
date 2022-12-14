use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

fn parse_input(input: &Vec<&str>) -> (HashSet<Coord>, isize) {
    let mut grid: HashSet<Coord> = HashSet::new();
    let mut max_y = 0;

    for line in input {
        let mut from: Option<Coord> = None;

        for part in line.split(" -> ") {
            let coords = part.split_once(',').unwrap();
            let to = Coord(coords.0.parse().unwrap(), coords.1.parse().unwrap());

            if let Some(from) = from {
                let range = if from.0 == to.0 {
                    from.1.min(to.1)..=from.1.max(to.1)
                } else {
                    from.0.min(to.0)..=from.0.max(to.0)
                };

                for coord in range.map(|i| {
                    if from.0 == to.0 {
                        Coord(from.0, i)
                    } else {
                        Coord(i, from.1)
                    }
                }) {
                    if coord.1 > max_y {
                        max_y = coord.1;
                    }
                    grid.insert(coord);
                }
            }

            from = Some(to);
        }
    }

    (grid, max_y)
}

pub fn part1(input: &Vec<&str>) -> usize {
    let (mut grid, max_y) = parse_input(input);

    let rocks_count = grid.len();
    let start = Coord(500, 0);

    'generator: loop {
        let mut sand_position = start.clone();

        'movements: loop {
            if sand_position.1 >= max_y {
                break 'generator;
            }

            for next in &[[0, 1], [-1, 1], [1, 1]] {
                let next_position = Coord(sand_position.0 + next[0], sand_position.1 + next[1]);
                if !grid.contains(&next_position) {
                    sand_position = next_position;
                    continue 'movements;
                }
            }

            break;
        }

        grid.insert(sand_position);
    }

    grid.len() - rocks_count
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (mut grid, max_y) = parse_input(input);

    let rocks_count = grid.len();
    let start = Coord(500, 0);
    let floor_y = max_y + 2;

    loop {
        if grid.contains(&start) {
            break;
        }

        let mut sand_position = start.clone();

        'movements: loop {
            for next in &[[0, 1], [-1, 1], [1, 1]] {
                let next_position = Coord(sand_position.0 + next[0], sand_position.1 + next[1]);
                if next_position.1 < floor_y && !grid.contains(&next_position) {
                    sand_position = next_position;
                    continue 'movements;
                }
            }

            break;
        }

        grid.insert(sand_position);
    }

    grid.len() - rocks_count
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 24)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 93)
    }
}
