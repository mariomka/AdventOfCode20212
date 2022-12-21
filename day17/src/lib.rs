use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord(isize, isize);

#[derive(Clone, Copy)]
enum RockType {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

#[derive(Clone)]
struct Rock {
    r#type: RockType,
    edge: Coord,
}

impl Rock {
    fn new(r#type: RockType, edge: Coord) -> Rock {
        Rock { r#type, edge }
    }

    fn coords(&self) -> Vec<Coord> {
        match self.r#type {
            RockType::Horizontal => vec![
                self.edge.clone(),
                Coord(self.edge.0 + 1, self.edge.1),
                Coord(self.edge.0 + 2, self.edge.1),
                Coord(self.edge.0 + 3, self.edge.1),
            ],
            RockType::Plus => vec![
                Coord(self.edge.0 + 1, self.edge.1 + 2),
                Coord(self.edge.0, self.edge.1 + 1),
                Coord(self.edge.0 + 1, self.edge.1 + 1),
                Coord(self.edge.0 + 2, self.edge.1 + 1),
                Coord(self.edge.0 + 1, self.edge.1),
            ],
            RockType::L => vec![
                Coord(self.edge.0 + 2, self.edge.1 + 2),
                Coord(self.edge.0 + 2, self.edge.1 + 1),
                self.edge.clone(),
                Coord(self.edge.0 + 1, self.edge.1),
                Coord(self.edge.0 + 2, self.edge.1),
            ],
            RockType::Vertical => vec![
                Coord(self.edge.0, self.edge.1 + 3),
                Coord(self.edge.0, self.edge.1 + 2),
                Coord(self.edge.0, self.edge.1 + 1),
                self.edge.clone(),
            ],
            RockType::Square => vec![
                Coord(self.edge.0, self.edge.1 + 1),
                Coord(self.edge.0 + 1, self.edge.1 + 1),
                self.edge.clone(),
                Coord(self.edge.0 + 1, self.edge.1),
            ],
        }
    }

    fn next(&self, diff: Coord) -> Self {
        Rock::new(
            self.r#type,
            Coord(self.edge.0 + diff.0, self.edge.1 + diff.1),
        )
    }
}

fn create_next_rock(i: usize, y: isize) -> Rock {
    let coord = Coord(2, y);

    match i % 5 {
        0 => Rock::new(RockType::Horizontal, coord),
        1 => Rock::new(RockType::Plus, coord),
        2 => Rock::new(RockType::L, coord),
        3 => Rock::new(RockType::Vertical, coord),
        4 => Rock::new(RockType::Square, coord),
        _ => panic!("Invalid rock"),
    }
}

pub fn part1(jet_pattern: &Vec<&str>) -> usize {
    let mut blocks = HashSet::new();
    let mut height = -1;
    let mut jet_pattern_index = 0;

    for i in 0..2022 {
        let mut rock = create_next_rock(i, height + 4);

        'outer: loop {
            // Move by jets
            let diff = match jet_pattern[jet_pattern_index % jet_pattern.len()] {
                ">" => Coord(1, 0),
                "<" => Coord(-1, 0),
                _ => panic!("Invalid jet pattern"),
            };

            let new_rock = rock.next(diff);
            let coords = new_rock.coords();
            let mut should_move = true;

            for coord in coords {
                if coord.0 < 0 || coord.0 > 6 || blocks.contains(&coord) {
                    should_move = false;
                    break;
                }

                if blocks.contains(&coord) {
                    break 'outer;
                }
            }

            if should_move {
                rock = new_rock;
            }

            jet_pattern_index += 1;

            // Move down
            let diff = Coord(0, -1);
            let new_rock = rock.next(diff);
            let coords = new_rock.coords();

            for coord in coords {
                if coord.1 < 0 || blocks.contains(&coord) {
                    break 'outer;
                }
            }

            rock = new_rock;
        }

        for coord in rock.coords() {
            if coord.1 > height {
                height = coord.1;
            }
            blocks.insert(coord);
        }
    }

    height as usize + 1
}

// pub fn part2(jet_pattern: &Vec<&str>) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use helpers::{input_lines, split_input};

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        split_input(input, "")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3068)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input()), 1514285714288)
    // }
}
