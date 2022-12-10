#[derive(Clone)]
struct DeviceVideoSystemState {
    register_x: isize,
    cycle: usize,
}

struct DeviceVideoSystem<'a> {
    state: DeviceVideoSystemState,
    instruction_pointer: usize,
    current_instruction: String,
    program: &'a Vec<&'a str>,
}

impl<'a> DeviceVideoSystem<'a> {
    fn init(program: &'a Vec<&'a str>) -> Self {
        Self {
            state: DeviceVideoSystemState {
                register_x: 1,
                cycle: 1,
            },
            instruction_pointer: 0,
            current_instruction: String::new(),
            program,
        }
    }

    fn run(&'a mut self) -> RunIterator {
        RunIterator { device: self }
    }

    fn run_next_instruction(&mut self) -> bool {
        if !self.current_instruction.is_empty() {
            let (_, increment) = self.current_instruction.split_once(" ").unwrap();
            self.state.register_x += increment.parse::<isize>().unwrap();
            self.current_instruction = String::new();
            self.state.cycle += 1;

            return true;
        }

        if self.instruction_pointer >= self.program.len() {
            return false;
        }

        let instruction = self.program[self.instruction_pointer];
        self.instruction_pointer += 1;

        if instruction == "noop" {
            self.state.cycle += 1;
            return true;
        }

        self.current_instruction = instruction.to_string();
        self.state.cycle += 1;

        true
    }
}

struct RunIterator<'a> {
    device: &'a mut DeviceVideoSystem<'a>,
}

impl<'a> Iterator for RunIterator<'a> {
    type Item = DeviceVideoSystemState;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.device.state.clone();

        if !self.device.run_next_instruction() {
            return None;
        }

        return Some(state);
    }
}

pub fn part1(input: &Vec<&str>) -> isize {
    DeviceVideoSystem::init(input)
        .run()
        .filter(|state| [20, 60, 100, 140, 180, 220].contains(&state.cycle))
        .map(|state| state.cycle as isize * state.register_x)
        .sum()
}

pub fn part2(input: &Vec<&str>) -> String {
    DeviceVideoSystem::init(input)
        .run()
        .map(|state| {
            let mut string = String::new();
            let pixel_x = ((state.cycle - 1) % 40) as usize;

            if state.cycle > 1 && pixel_x == 0 {
                string += "\n";
            }

            let sprite_range = state.register_x - 1..=state.register_x + 1;

            string += if sprite_range.contains(&(pixel_x as isize)) {
                "#"
            } else {
                "."
            };

            string
        })
        .collect()
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
