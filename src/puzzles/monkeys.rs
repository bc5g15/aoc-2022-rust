use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Operation {
    Plus (u64),
    Multiply (u64),
    Square
}

// type Operation = (Operand, u64);

fn build_operation(input: Vec<&str>) -> Operation {
    match input.get(1).unwrap() {
        &"+" => {
            let value: u64 = input.get(0).unwrap().parse().unwrap();
            Operation::Plus(value)
        },
        &"*" => {
            let value = input.get(0).unwrap();
            if value == &"old" {
                return Operation::Square
            }
            let value: u64 = value.parse().unwrap();
            Operation::Multiply(value)
        }
        n => panic!("Unrecognised build op {n}")
    }

    // let value: u64 = input.get(0).unwrap().parse().unwrap();
    // let op = match input.get(1).unwrap() {
    //     &"+" => Operand::Plus,
    //     &"*" => Operand::Multiply,
    //     n => panic!("Unrecognised operand {n}")
    // };
    // (op, value)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    div_check: u64,
    truth_index: u64,
    false_index: u64,
    worried: bool
}

impl Monkey {
    fn throw(&mut self, max_worry: u64) -> (u64, u64) {
        let mut item = self.items.pop_front().unwrap();

        // Apply operator to worry level
        item = match self.operation {
            Operation::Plus(value) => item + value,
            Operation::Multiply(value) => item * value,
            Operation::Square => item * item
        };

        if !self.worried {
            // Decrease worry level because you are chill
            item = item / 3;
        } else {
            // I'm pretty sure this is how clock maths works?
            item = item % max_worry
        }

        let index = if item % self.div_check == 0 {
            self.truth_index
        } else {
            self.false_index
        };
        (item, index)
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn parse_monkey(input: &[&str], worried: bool) -> Monkey {
    // First line is number, skip
    let mut iter = input.iter().skip(1);

    let items: VecDeque<u64> = iter.next().unwrap().trim().chars()
        .filter(|c| c.is_numeric() || c.is_whitespace()).collect::<String>()
        .trim().split(" ").map(|v| v.parse().unwrap()).collect();

    let op_string: Vec<&str> = iter.next().unwrap().trim().split(" ").collect();
    let relevant_parts: Vec<&str> = op_string.iter().rev().take(2).map(|s| *s).collect();
    let operation = build_operation(relevant_parts);

    let divisibility: u64 = iter.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();
    let truth_index: u64 = iter.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();
    let false_index: u64 = iter.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();

    Monkey {
        items,
        operation,
        div_check: divisibility,
        truth_index,
        false_index,
        worried
    }
}

fn parse_monkeys(input: &String, worried: bool) -> Vec<Monkey> {
    let vectored: Vec<&str> = input.trim().lines().collect();
    let grouped: Vec<&[&str]> = vectored.split(|s| *s == "").collect();

    grouped.iter().map(|m| {
        let ms = *m;
        parse_monkey(ms, worried)
    }).collect()
}

pub fn monkey_business(input: &String, worried: bool) -> u64 {
    let mut monkeys = parse_monkeys(input, worried);
    let mut throw_count: Vec<u64> = (0..monkeys.len()).map(|_| 0).collect();

    let max_worry_level: u64 = monkeys.iter().map(|m| m.div_check).product();

    let max_rounds = if worried { 10000 } else { 20 };

    for _ in 0..max_rounds {
        for i in 0..monkeys.len() {
            let mut thrown_items: VecDeque<(u64, u64)> = VecDeque::new();
            let monkey = monkeys.get_mut(i).unwrap();
            throw_count[i] += monkey.items.len() as u64;
            while monkey.items.len() > 0 {
                thrown_items.push_back(monkey.throw(max_worry_level));
            }
            while thrown_items.len() > 0 {
                let (item, index) = thrown_items.pop_front().unwrap();
                monkeys.get_mut(index as usize).unwrap().catch(item);
            }
        }
    }
    throw_count.sort();
    throw_count.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
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
    If false: throw to monkey 1
    ".to_string();

        assert_eq!(monkey_business(&input, false), 10605);

        assert_eq!(monkey_business(&input, true), 2713310158)
    }
}
