struct Symbol(i8);

impl From<char> for Symbol {
    fn from(c: char) -> Symbol {
        match c {
            'A' | 'X' => Symbol(-1),
            'B' | 'Y' => Symbol(0),
            'C' | 'Z' => Symbol(1),
            _ => unreachable!(),
        }
    }
}

struct Input {
    their_choice: Symbol,
    unknown: Symbol,
}

fn from_str(s: &str) -> Input {
    Input {
        their_choice: Symbol::from(s.chars().next().unwrap()),
        unknown: Symbol::from(s.chars().nth(2).unwrap()),
    }
}

fn transform(i: i8) -> i8 {
    match i {
        1 | -2 => 1,
        -1 | 2 => -1,
        _ => 0,
    }
}

fn choice(input: Input) -> u64 {
    let outcome = transform(input.unknown.0 - input.their_choice.0);
    (input.unknown.0 + 2) as u64 + 3 * (outcome + 1) as u64
}

fn outcome(input: Input) -> u64 {
    let choice = transform(input.unknown.0 + input.their_choice.0);
    (choice + 2) as u64 + 3 * (input.unknown.0 + 1) as u64
}

fn main() {
    let input = include_str!("input");
    println!("{}", input.lines().map(from_str).map(choice).sum::<u64>());
    println!("{}", input.lines().map(from_str).map(outcome).sum::<u64>());
}

#[cfg(test)]
mod tests {
    use crate::{choice, from_str, outcome};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(INPUT.lines().map(from_str).map(choice).sum::<u64>(), 15);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(INPUT.lines().map(from_str).map(outcome).sum::<u64>(), 12);
    }
}
