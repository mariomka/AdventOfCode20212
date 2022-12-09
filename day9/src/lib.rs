use std::collections::HashSet;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn distance(&self, other: &Coord) -> isize {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32)
            .sqrt()
            .round() as isize
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn direction_vector_from_direction(direction: char) -> Coord {
    match direction {
        'U' => Coord::new(0, 1),
        'R' => Coord::new(1, 0),
        'D' => Coord::new(0, -1),
        'L' => Coord::new(-1, 0),
        _ => panic!("Unknown direction: {}", direction),
    }
}

fn direction_vector_from_two_coords(a: Coord, b: Coord) -> Coord {
    Coord::new(
        match a.x - b.x {
            x if x > 0 => 1,
            x if x < 0 => -1,
            _ => 0,
        },
        match a.y - b.y {
            x if x > 0 => 1,
            x if x < 0 => -1,
            _ => 0,
        },
    )
}

fn next_knot_position(knot: Coord, previous: Coord) -> Coord {
    if knot.distance(&previous) > 1 {
        return knot + direction_vector_from_two_coords(previous, knot);
    }

    knot
}

pub fn part1(input: &Vec<&str>) -> usize {
    let mut head_position: Coord = Coord::new(0, 0);
    let mut tail_position: Coord = Coord::new(0, 0);
    let mut visited: HashSet<Coord> = HashSet::new();

    for line in input {
        let (direction, steps) = line.split_once(" ").unwrap();
        let direction_vector = direction_vector_from_direction(direction.chars().next().unwrap());

        for _ in 0..steps.parse().unwrap() {
            head_position += direction_vector;
            tail_position = next_knot_position(tail_position, head_position);

            visited.insert(tail_position);
        }
    }

    visited.len()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut knots_positions: Vec<Coord> = vec![Coord::new(0, 0); 10];
    let mut visited: HashSet<Coord> = HashSet::new();

    for line in input {
        let (direction, steps) = line.split_once(" ").unwrap();
        let direction_vector = direction_vector_from_direction(direction.chars().next().unwrap());

        for _ in 0..steps.parse().unwrap() {
            knots_positions[0] += direction_vector;

            for i in 1..knots_positions.len() {
                knots_positions[i] = next_knot_position(knots_positions[i], knots_positions[i - 1]);
            }

            visited.insert(knots_positions[9]);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part1(&input_lines(input)), 13)
    }

    #[test]
    fn test_part2() {
        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(&input_lines(input)), 36)
    }
}
