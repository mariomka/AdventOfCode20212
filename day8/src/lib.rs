use helpers::Grid;

pub fn part1(input: &Grid<usize>) -> usize {
    input
        .iter()
        .filter(|(coords, &height)| {
            // left
            (0..coords.0)
                .all(|x| *input.get((x, coords.1)) < height)

            // right
            || (coords.0 + 1..input.size.0)
                .all(|x| *input.get((x, coords.1)) < height)

            // top
            || (0..coords.1)
                .all(|y| *input.get((coords.0, y)) < height)

            // bottom
            || (coords.1 + 1..input.size.1)
                .all(|y| *input.get((coords.0, y)) < height)
        })
        .count()
}

pub fn part2(input: &Grid<usize>) -> usize {
    input
        .iter()
        .map(|(coords, &height)| {
            let mut score = 1;
            let mut viewing_distance = 0;

            // left
            for x in (0..coords.0).rev() {
                viewing_distance += 1;
                if *input.get((x, coords.1)) >= height {
                    break;
                }
            }
            score *= viewing_distance;
            viewing_distance = 0;

            // right
            for x in coords.0 + 1..input.size.0 {
                viewing_distance += 1;
                if *input.get((x, coords.1)) >= height {
                    break;
                }
            }
            score *= viewing_distance;
            viewing_distance = 0;

            // top
            for y in (0..coords.1).rev() {
                viewing_distance += 1;
                if *input.get((coords.0, y)) >= height {
                    break;
                }
            }
            score *= viewing_distance;
            viewing_distance = 0;

            // bottom
            for y in coords.1 + 1..input.size.1 {
                viewing_distance += 1;
                if *input.get((coords.0, y)) >= height {
                    break;
                }
            }
            score *= viewing_distance;

            return score;
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::input_grid;

    use super::*;

    fn input() -> Grid<usize> {
        let input = "\
30373
25512
65332
33549
35390";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 21)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 8)
    }
}
