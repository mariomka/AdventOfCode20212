use std::str::FromStr;

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn for_match_result(other: &Shape, match_result: MatchResult) -> Shape {
        match match_result {
            MatchResult::Win => match other {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            MatchResult::Loss => match other {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            MatchResult::Draw => other.clone(),
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Shape, Self::Err> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn score(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loss => 0,
        }
    }
}

impl FromStr for MatchResult {
    type Err = ();

    fn from_str(input: &str) -> Result<MatchResult, Self::Err> {
        match input {
            "X" => Ok(MatchResult::Loss),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
        }
    }
}

struct Match {
    opponent_shape: Shape,
    my_shape: Shape,
}

impl Match {
    fn compare(a: &Shape, b: &Shape) -> MatchResult {
        if a == b {
            return MatchResult::Draw;
        }

        if (a == &Shape::Rock && b == &Shape::Scissors)
            || (a == &Shape::Paper && b == &Shape::Rock)
            || (a == &Shape::Scissors && b == &Shape::Paper)
        {
            return MatchResult::Win;
        }

        return MatchResult::Loss;
    }

    fn my_score(&self) -> usize {
        self.my_shape.score() + Match::compare(&self.my_shape, &self.opponent_shape).score()
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            Match {
                opponent_shape: split.next().unwrap().parse().unwrap(),
                my_shape: split.next().unwrap().parse().unwrap(),
            }
        })
        .map(|m| m.my_score())
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let opponent_shape: Shape = split.next().unwrap().parse().unwrap();
            let my_shape: Shape =
                Shape::for_match_result(&opponent_shape, split.next().unwrap().parse().unwrap());

            Match {
                opponent_shape,
                my_shape,
            }
        })
        .map(|m| m.my_score())
        .sum()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "A Y
B X
C Z";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 12)
    }
}
