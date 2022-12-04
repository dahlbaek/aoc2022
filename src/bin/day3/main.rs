use std::collections::HashSet;

fn get_priority(b: u8) -> u64 {
    u64::from(match b {
        65..=90 => b - 38,
        97..=122 => b - 96,
        _ => panic!("Found unrecognized byte {}", b),
    })
}

fn find_1(input: &str) -> u64 {
    let mut res = 0;
    for sack in input.lines() {
        let len = sack.len() / 2;
        let mut chars = sack.bytes();
        let mut first = HashSet::with_capacity(len);
        for _ in 0..len {
            first.insert(chars.next().unwrap());
        }
        let shared = chars.find(|el| first.contains(el));
        res += get_priority(shared.unwrap());
    }
    res
}

fn find_2(input: &str) -> u64 {
    let mut res = 0;
    let mut lines = input.lines().map(|l| l.bytes().collect::<HashSet<_>>());
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let shared = a.into_iter().find(|el| b.contains(el) && c.contains(el));
        res += get_priority(shared.unwrap());
    }
    res
}

fn main() {
    let input = include_str!("input");
    println!("{}", find_1(input));
    println!("{}", find_2(input));
}

#[cfg(test)]
mod tests {
    use crate::{find_1, find_2};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(find_1(INPUT), 157);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find_2(INPUT), 70);
    }
}
