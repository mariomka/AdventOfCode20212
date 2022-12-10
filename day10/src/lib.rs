pub fn part1(input: &Vec<&str>) -> isize {
    let mut register_x: isize = 1;
    let mut index: usize = 0;
    let mut cycle: usize = 1;
    let mut current_instruction = "";
    let mut signal_strength = 0;

    loop {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strength += cycle as isize * register_x;
        }

        if cycle == 220 || input.len() == index {
            break;
        }

        if !current_instruction.is_empty() {
            let (_, increment) = current_instruction.split_once(" ").unwrap();
            register_x += increment.parse::<isize>().unwrap();
            current_instruction = "";

            cycle += 1;
            continue;
        }

        let instruction = input[index];
        index += 1;

        if instruction == "noop" {
            cycle += 1;
            continue;
        }

        current_instruction = instruction;
        cycle += 1;
    }

    signal_strength
}

pub fn part2(input: &Vec<&str>) -> String {
    let mut register_x: isize = 1;
    let mut index: usize = 0;
    let mut cycle: usize = 1;
    let mut current_instruction = "";
    let mut crt_pixels = "".to_string();

    loop {
        if input.len() == index {
            break;
        }

        let pixel_x = ((cycle - 1) % 40) as usize;

        if cycle > 1 && pixel_x == 0 {
            crt_pixels += "\n";
        }

        crt_pixels += if (register_x - 1..=register_x + 1).contains(&(pixel_x as isize)) {
            "#"
        } else {
            "."
        };

        if !current_instruction.is_empty() {
            let (_, increment) = current_instruction.split_once(" ").unwrap();
            register_x += increment.parse::<isize>().unwrap();
            current_instruction = "";

            cycle += 1;
            continue;
        }

        let instruction = input[index];
        index += 1;

        if instruction == "noop" {
            cycle += 1;
            continue;
        }

        current_instruction = instruction;
        cycle += 1;
    }

    crt_pixels
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 13140)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&input()),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
