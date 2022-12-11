trait Operation: OperationClone {
    fn execute(&self, old: usize) -> usize;
}

trait OperationClone {
    fn clone_box(&self) -> Box<dyn Operation>;
}

impl<T> OperationClone for T
where
    T: 'static + Operation + Clone,
{
    fn clone_box(&self) -> Box<dyn Operation> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Operation> {
    fn clone(&self) -> Box<dyn Operation> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Add {
    value: usize,
}

impl Operation for Add {
    fn execute(&self, old: usize) -> usize {
        old + self.value
    }
}

#[derive(Clone)]
struct Multiply {
    value: usize,
}

impl Operation for Multiply {
    fn execute(&self, old: usize) -> usize {
        old * self.value
    }
}

#[derive(Clone)]
struct MultiplyOld {}

impl Operation for MultiplyOld {
    fn execute(&self, old: usize) -> usize {
        old * old
    }
}

#[derive(Clone)]
struct Test {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Operation>,
    test: Test,
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            operation: self.operation.clone(),
            test: self.test.clone(),
        }
    }
}

fn parse_monkeys(input: &Vec<&str>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut input_iter = input.iter();

    while let Some(mut line) = input_iter.next() {
        line = input_iter.next().unwrap();
        let items: Vec<usize> = line["Starting items: ".len()..]
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        line = input_iter.next().unwrap();
        let operation_split = line["Operation: new = old ".len()..]
            .split(" ")
            .collect::<Vec<&str>>();

        let operation = if operation_split[1] == "old" {
            Box::new(MultiplyOld {})
        } else {
            let operation_value = operation_split[1].parse::<usize>().unwrap();
            match operation_split[0] {
                "+" => Box::new(Add {
                    value: operation_value,
                }) as Box<dyn Operation>,
                "*" => Box::new(Multiply {
                    value: operation_value,
                }) as Box<dyn Operation>,
                _ => panic!("Unknown operation"),
            }
        };

        line = input_iter.next().unwrap();
        let divisible_by = line["Test: divisible by ".len()..]
            .parse::<usize>()
            .unwrap();

        line = input_iter.next().unwrap();
        let if_true = line["If true: throw to monkey ".len()..]
            .parse::<usize>()
            .unwrap();

        line = input_iter.next().unwrap();
        let if_false = line["If false: throw to monkey ".len()..]
            .parse::<usize>()
            .unwrap();

        let test = Test {
            divisible_by,
            if_true,
            if_false,
        };

        monkeys.push(Monkey {
            items,
            operation,
            test,
        });
    }

    monkeys
}

fn calc_monkey_business<F>(monkeys: Vec<Monkey>, rounds: usize, reducer: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut inspection_count = vec![0; monkeys.len()];

    let mut current_monkeys = monkeys.clone();
    for _ in 0..rounds {
        for i in 0..current_monkeys.len() {
            let mut next_monkeys = current_monkeys.clone();
            let monkey = current_monkeys.get(i).unwrap();

            for item in monkey.items.iter() {
                inspection_count[i] += 1;

                let new_item = reducer(monkey.operation.execute(*item));

                if new_item % monkey.test.divisible_by == 0 {
                    next_monkeys[monkey.test.if_true].items.push(new_item);
                } else {
                    next_monkeys[monkey.test.if_false].items.push(new_item);
                }
            }

            next_monkeys[i].items = vec![];
            current_monkeys = next_monkeys;
        }
    }

    inspection_count.sort_unstable();
    inspection_count.iter().rev().take(2).product()
}

pub fn part1(input: &Vec<&str>) -> usize {
    let monkeys = parse_monkeys(input);

    calc_monkey_business(monkeys, 20, |value| value / 3)
}

pub fn part2(input: &Vec<&str>) -> usize {
    let monkeys = parse_monkeys(input);
    let common_multiple: usize = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    calc_monkey_business(monkeys, 10_000, |value| value % common_multiple)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 10605)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2713310158)
    }
}
