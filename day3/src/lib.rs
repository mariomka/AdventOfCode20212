use std::collections::HashSet;

fn priority(item: &char) -> usize {
    let code = *item as usize;

    if code > 96 {
        return code - 96;
    }

    code - 65 + 27
}

pub fn part1(input: &Vec<&str>) -> usize {
    input.iter()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);
            let left: HashSet<char> = compartments.0.chars().collect();
            let right: HashSet<char> = compartments.1.chars().collect();

            priority(left.intersection(&right).last().unwrap())
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .chunks(3)
        .map(|chunk| {
            let set_a: Vec<char> = chunk[0].chars().collect();
            let set_b: Vec<char> = chunk[1].chars().collect();
            let set_c: Vec<char> = chunk[2].chars().collect();

            let sets = [&set_a, &set_b, &set_c];
            let intersection = set_a.iter().filter(|k| sets.iter().all(|s| s.contains(k)));

            priority(intersection.last().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 157)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 70)
    }
}
