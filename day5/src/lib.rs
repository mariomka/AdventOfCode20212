use std::collections::VecDeque;

fn parse_input(input: &Vec<&str>) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut movements: Vec<(usize, usize, usize)> = Vec::new();

    for line in input {
        if line.trim().is_empty() {
            continue;
        }

        // is a stack
        if line.trim().starts_with("[") {
            for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
                if index >= stacks.len() {
                    stacks.push(VecDeque::new());
                }

                if char != ' ' {
                    stacks[index].push_back(char);
                }
            }
        }

        //  is a movement
        if line.trim().starts_with("m") {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            movements.push((
                split[1].parse().unwrap(),
                split[3].parse::<usize>().unwrap() - 1,
                split[5].parse::<usize>().unwrap() - 1,
            ));
        }
    }

    (stacks, movements)
}

pub fn part1(input: &Vec<&str>) -> String {
    let (mut stacks, movements) = parse_input(input);

    for (count, from, to) in movements {
        let from_stack = stacks.get_mut(from).unwrap();
        let move_crates: Vec<char> = from_stack.drain(0..count).collect();

        let to_stack = stacks.get_mut(to).unwrap();
        for move_crate in move_crates {
            to_stack.push_front(move_crate);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.front().unwrap())
        .collect::<String>()
}

pub fn part2(input: &Vec<&str>) -> String {
    let (mut stacks, movements) = parse_input(input);

    for (count, from, to) in movements {
        let from_stack = stacks.get_mut(from).unwrap();
        let move_crates: Vec<char> = from_stack.drain(0..count).rev().collect();

        let to_stack = stacks.get_mut(to).unwrap();
        for move_crate in move_crates {
            to_stack.push_front(move_crate);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.front().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines_raw;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        input_lines_raw(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), "CMZ")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), "MCD")
    }
}
