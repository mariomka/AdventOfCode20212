use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Cube { x, y, z }
    }

    fn neighbors(&self) -> Vec<Cube> {
        let mut neighbors = Vec::new();

        for (x, y, z) in &[
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
        ] {
            neighbors.push(Cube::new(self.x + x, self.y + y, self.z + z));
        }

        neighbors
    }
}

fn sides_exposed(cubes: &HashSet<Cube>) -> usize {
    let mut total_sides_exposed = 0;

    for cube in cubes.iter() {
        let mut sides_exposed = 0;

        for neighbor in cube.neighbors() {
            if !cubes.contains(&neighbor) {
                sides_exposed += 1;
            }
        }

        total_sides_exposed += sides_exposed;
    }

    total_sides_exposed
}

pub fn part1(input: &Vec<&str>) -> usize {
    let mut cubes = HashSet::new();

    for line in input {
        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<isize>().unwrap();
        let y = parts[1].parse::<isize>().unwrap();
        let z = parts[2].parse::<isize>().unwrap();
        cubes.insert(Cube::new(x, y, z));
    }

    sides_exposed(&cubes)
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut cubes = HashSet::new();
    let mut max_x = 0;
    let mut min_x = isize::MAX;
    let mut max_y = 0;
    let mut min_y = isize::MAX;
    let mut max_z = 0;
    let mut min_z = isize::MAX;

    for line in input {
        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<isize>().unwrap();
        let y = parts[1].parse::<isize>().unwrap();
        let z = parts[2].parse::<isize>().unwrap();
        cubes.insert(Cube::new(x, y, z));

        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }
        if z > max_z {
            max_z = z;
        }
        if z < min_z {
            min_z = z;
        }
    }

    let mut sides_exposed = sides_exposed(&cubes);
    let mut global_visited = HashSet::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            'outer: for z in min_z..=max_z {
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                let cube = Cube::new(x, y, z);

                let mut path_sides_occupied = 0;

                if cubes.contains(&cube) || global_visited.contains(&cube) {
                    continue;
                }

                queue.push_back(cube);

                while let Some(cube) = queue.pop_front() {
                    if visited.contains(&cube) {
                        continue;
                    }

                    visited.insert(cube);

                    let mut sides_occuppied = 0;

                    for neighbor in cube.neighbors() {
                        if global_visited.contains(&neighbor)
                            || neighbor.x < min_x
                            || neighbor.y < min_y
                            || neighbor.z < min_z
                            || neighbor.x > max_x
                            || neighbor.y > max_y
                            || neighbor.z > max_z
                        {
                            continue 'outer;
                        }

                        if cubes.contains(&neighbor) {
                            sides_occuppied += 1;
                        } else {
                            if !visited.contains(&neighbor) {
                                queue.push_back(neighbor);
                            }
                        }
                    }

                    path_sides_occupied += sides_occuppied;
                }

                global_visited.extend(visited);
                sides_exposed -= path_sides_occupied;
            }
        }
    }

    sides_exposed
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 64)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 58)
    }
}
