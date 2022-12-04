use std::ops::Range;

fn parse_pair(line: &&&str) -> (Range<usize>, Range<usize>) {
    let pair: Vec<Range<usize>> = line
        .split(",")
        .map(|section| {
            let section = section
                .split("-")
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            Range {
                start: section[0],
                end: section[1],
            }
        })
        .collect();

    (pair[0].to_owned(), pair[1].to_owned())
}

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .into_iter()
        .filter(|line| {
            let pair = parse_pair(line);

            (pair.0.start <= pair.1.start && pair.0.end >= pair.1.end)
                || (pair.0.start >= pair.1.start && pair.0.end <= pair.1.end)
        })
        .count()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .into_iter()
        .filter(|line| {
            let pair = parse_pair(line);

            pair.0.start <= pair.1.end && pair.1.start <= pair.0.end
        })
        .count()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 4)
    }
}
