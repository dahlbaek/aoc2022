#![feature(test)]

extern crate test;

fn read(b: u8) -> i8 {
    match b {
        b'A' | b'B' | b'C' => b as i8 - b'B' as i8,
        b'X' | b'Y' | b'Z' => b as i8 - b'Y' as i8,
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
        0 => 0,
        _ => panic!(),
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
    use test::Bencher;

    use crate::{choice, outcome, prepare};

    const INPUT: &'static str = include_str!("bench-input");
    const TINPUT: &'static str = include_str!("test-input");

    fn read_old(b: u8) -> i8 {
        match b {
            b'A' | b'X' => -1,
            b'B' | b'Y' => 0,
            b'C' | b'Z' => 1,
            _ => panic!("Found unrecognized byte {}", b),
        }
    }

    fn prepare_old(s: &str) -> (i8, i8) {
        let mut bytes = s.bytes();
        (
            read_old(bytes.next().unwrap()),
            read_old(bytes.nth(1).unwrap()),
        )
    }

    const ARR: [i8; 26] = [
        -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1,
    ];

    fn read_arr(b: u8) -> i8 {
        ARR[(b - b'A') as usize]
    }

    fn prepare_arr(s: &str) -> (i8, i8) {
        let mut bytes = s.bytes();
        (
            read_arr(bytes.next().unwrap()),
            read_arr(bytes.nth(1).unwrap()),
        )
    }

    #[test]
    fn it_works_1() {
        assert_eq!(TINPUT.lines().map(prepare).map(choice).sum::<u64>(), 15);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(TINPUT.lines().map(prepare).map(outcome).sum::<u64>(), 12);
    }

    #[test]
    fn it_works_old_1() {
        assert_eq!(TINPUT.lines().map(prepare_old).map(choice).sum::<u64>(), 15);
    }

    #[test]
    fn it_works_old_2() {
        assert_eq!(
            TINPUT.lines().map(prepare_old).map(outcome).sum::<u64>(),
            12
        );
    }

    #[test]
    fn it_works_arr_1() {
        assert_eq!(TINPUT.lines().map(prepare_arr).map(choice).sum::<u64>(), 15);
    }

    #[test]
    fn it_works_arr_2() {
        assert_eq!(
            TINPUT.lines().map(prepare_arr).map(outcome).sum::<u64>(),
            12
        );
    }

    #[bench]
    fn bench_choice(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare).map(choice).sum::<u64>());
    }

    #[bench]
    fn bench_outcome(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare).map(outcome).sum::<u64>());
    }

    #[bench]
    fn bench_choice_old(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare_old).map(choice).sum::<u64>());
    }

    #[bench]
    fn bench_outcome_old(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare_old).map(outcome).sum::<u64>());
    }

    #[bench]
    fn bench_choice_arr(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare_arr).map(choice).sum::<u64>());
    }

    #[bench]
    fn bench_outcome_arr(b: &mut Bencher) {
        b.iter(|| INPUT.lines().map(prepare_arr).map(outcome).sum::<u64>());
    }
}
