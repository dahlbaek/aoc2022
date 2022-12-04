fn read(b: u8) -> i8 {
    match b {
        b'A' | b'X' => -1,
        b'B' | b'Y' => 0,
        b'C' | b'Z' => 1,
        _ => panic!("Found unrecognized byte {}", b),
    }
}

fn prepare(s: &str) -> (i8, i8) {
    let mut bytes = s.bytes();
    (read(bytes.next().unwrap()), read(bytes.nth(1).unwrap()))
}

fn transform(i: i8) -> i8 {
    match i {
        1 | -2 => 1,
        -1 | 2 => -1,
        _ => 0,
    }
}

fn choice((their_choice, choice): (i8, i8)) -> u64 {
    let outcome = transform(choice - their_choice);
    (choice + 2) as u64 + 3 * (outcome + 1) as u64
}

fn outcome((their_choice, outcome): (i8, i8)) -> u64 {
    let choice = transform(outcome + their_choice);
    (choice + 2) as u64 + 3 * (outcome + 1) as u64
}

fn main() {
    let input = include_str!("input");
    println!("{}", input.lines().map(prepare).map(choice).sum::<u64>());
    println!("{}", input.lines().map(prepare).map(outcome).sum::<u64>());
}

#[cfg(test)]
mod tests {
    use crate::{choice, outcome, prepare};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(INPUT.lines().map(prepare).map(choice).sum::<u64>(), 15);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(INPUT.lines().map(prepare).map(outcome).sum::<u64>(), 12);
    }
}
