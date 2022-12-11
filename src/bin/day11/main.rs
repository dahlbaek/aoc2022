use std::{collections::VecDeque, error::Error, str::FromStr};

#[derive(Debug, PartialEq)]
enum Operation {
    Multiplication(u64),
    Addition(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    true_throw: usize,
    false_throw: usize,
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);

        let items = lines.next().ok_or("Starting items missing")?[18..]
            .split(", ")
            .map(u64::from_str)
            .collect::<Result<VecDeque<_>, _>>()?;
        let ops = lines.next().ok_or("Operation missing")?;
        let operation = match (ops.as_bytes()[23], &ops[25..]) {
            (b'*', "old") => Operation::Square,
            (b'*', i) => Operation::Multiplication(i.parse()?),
            (b'+', i) => Operation::Addition(i.parse()?),
            b => panic!("Unknown op {:?}", b),
        };
        let test = lines.next().ok_or("Test missing")?[21..].parse()?;
        let true_throw = lines.next().ok_or("If true missing")?[29..].parse()?;
        let false_throw = lines.next().ok_or("If false missing")?[30..].parse()?;

        Ok(Monkey {
            items,
            operation,
            test,
            true_throw,
            false_throw,
        })
    }
}

fn find(s: &str, rounds: u64, divide: u64) -> usize {
    let mut monkeys = s
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let space = monkeys.iter().fold(1, |acc, m| acc * m.test);

    let mut monkey_business = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for j in 0..monkeys.len() {
            while let Some(item) = monkeys[j].items.pop_front() {
                monkey_business[j] += 1;
                let new_value = match monkeys[j].operation {
                    Operation::Addition(i) => ((item + i) / divide) % space,
                    Operation::Multiplication(i) => ((item * i) / divide) % space,
                    Operation::Square => (item * item / divide) % space,
                };
                if new_value % monkeys[j].test == 0 {
                    let throw_to = monkeys[j].true_throw;
                    monkeys[throw_to].items.push_back(new_value)
                } else {
                    let throw_to = monkeys[j].false_throw;
                    monkeys[throw_to].items.push_back(new_value)
                }
            }
        }
    }

    let top2 = monkey_business.into_iter().fold((0, 0), |mut acc, el| {
        if el > acc.0 {
            acc.1 = acc.0;
            acc.0 = el;
        } else if el > acc.1 {
            acc.1 = el;
        }
        acc
    });

    top2.0 * top2.1
}

fn main() {
    let input = include_str!("input");
    println!("{}", find(input, 20, 3));
    println!("{}", find(input, 10000, 1));
}

#[cfg(test)]
mod tests {

    use crate::find;

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(find(INPUT, 20, 3), 10605);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find(INPUT, 10000, 1), 2713310158);
    }
}
