use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Packet(String);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match compare(self.0.clone(), other.0.clone()) {
            CompareResult::RightOrder => Some(Ordering::Less),
            CompareResult::Continue => Some(Ordering::Equal),
            CompareResult::WrongOrder => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug, PartialEq)]
enum CompareResult {
    RightOrder,
    WrongOrder,
    Continue,
}

fn split(input: String) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }

    let mut depth = 0;
    let mut split = vec![String::new()];

    for char in input.chars() {
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => split.push(String::new()),
            _ => {}
        }

        if char != ',' || depth != 0 {
            split.last_mut().unwrap().push(char);
        }
    }

    split
}

fn compare(left: String, right: String) -> CompareResult {
    if left.starts_with("[") || right.starts_with("[") {
        let left = if left.starts_with("[") {
            left[1..left.len() - 1].to_string()
        } else {
            left
        };

        let right = if right.starts_with("[") {
            right[1..right.len() - 1].to_string()
        } else {
            right
        };

        let left_split = split(left);
        let right_split = split(right);

        for i in 0..left_split.len() {
            let left = left_split.get(i).unwrap();

            if i >= right_split.len() {
                return CompareResult::WrongOrder;
            }
            let right = right_split.get(i).unwrap();

            match compare(left.to_string(), right.to_string()) {
                CompareResult::RightOrder => return CompareResult::RightOrder,
                CompareResult::WrongOrder => return CompareResult::WrongOrder,
                CompareResult::Continue => continue,
            }
        }

        if left_split.len() < right_split.len() {
            return CompareResult::RightOrder;
        }
        return CompareResult::Continue;
    }

    return match left.parse::<isize>().unwrap() - right.parse::<isize>().unwrap() {
        res if res > 0 => CompareResult::WrongOrder,
        res if res < 0 => CompareResult::RightOrder,
        _ => CompareResult::Continue,
    };
}

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .chunks(2)
        .enumerate()
        .filter(|&(_, pair)| {
            let left = pair[0].to_string();
            let right = pair[1].to_string();

            compare(left, right) == CompareResult::RightOrder
        })
        .map(|(index, _)| index + 1)
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut input = input.clone();
    input.push("[[2]]");
    input.push("[[6]]");

    let mut input = input
        .iter()
        .map(|packet| Packet(packet.to_string()))
        .collect::<Vec<Packet>>();
    input.sort_unstable();

    input
        .iter()
        .enumerate()
        .filter(|(_, packet)| packet.0 == "[[2]]" || packet.0 == "[[6]]")
        .map(|(index, _)| index + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut input()), 140)
    }
}
