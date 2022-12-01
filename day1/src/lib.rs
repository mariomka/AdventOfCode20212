pub fn part1(input: &Vec<&str>) -> usize {
    let elves = parse_elves(input);
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let elves = parse_elves(input);
    let mut elf_calories: Vec<usize> = elves.iter().map(|elf| elf.iter().sum()).collect();
    elf_calories.sort_unstable();
    elf_calories.iter().rev().take(3).sum()
}

fn parse_elves(input: &Vec<&str>) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|line| {
            line.split("\n")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use helpers::split_input;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        split_input(input, "\n\n")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 24000)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 45000)
    }
}
