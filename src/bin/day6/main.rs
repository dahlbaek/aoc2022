#![feature(test)]

extern crate test;

fn build_bitset(bytes: &[u8]) -> u64 {
    bytes
        .into_iter()
        .fold(0, |bitset, byte| bitset | (1 << (byte - b'a')))
}

fn first_marker(l: &str, size: u32) -> usize {
    let usize_size = size as usize;
    usize_size
        + l.as_bytes()
            .windows(usize_size)
            .enumerate()
            .find(|(_, bytes)| build_bitset(bytes).count_ones() == size)
            .unwrap()
            .0
}

fn main() {
    let input = include_str!("input");
    println!("{}", first_marker(input, 4));
    println!("{}", first_marker(input, 14));
}

#[cfg(test)]
mod tests {
    use crate::first_marker;
    use std::collections::HashSet;
    use test::Bencher;

    const INPUT: &'static str = include_str!("input");
    const INPUT1: &'static str = include_str!("test-input-1");
    const INPUT2: &'static str = include_str!("test-input-2");
    const INPUT3: &'static str = include_str!("test-input-3");
    const INPUT4: &'static str = include_str!("test-input-4");
    const INPUT5: &'static str = include_str!("test-input-5");

    fn first_marker_hashset(l: &str, size: usize) -> usize {
        size + l
            .as_bytes()
            .windows(size)
            .enumerate()
            .find(|(_, bytes)| HashSet::<&u8>::from_iter(bytes.iter()).len() == size)
            .unwrap()
            .0
    }

    #[test]
    fn it_works_1() {
        assert_eq!(first_marker(INPUT1, 4), 7);
        assert_eq!(first_marker(INPUT2, 4), 5);
        assert_eq!(first_marker(INPUT3, 4), 6);
        assert_eq!(first_marker(INPUT4, 4), 10);
        assert_eq!(first_marker(INPUT5, 4), 11);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(first_marker(INPUT1, 14), 19);
        assert_eq!(first_marker(INPUT2, 14), 23);
        assert_eq!(first_marker(INPUT3, 14), 23);
        assert_eq!(first_marker(INPUT4, 14), 29);
        assert_eq!(first_marker(INPUT5, 14), 26);
    }

    #[test]
    fn it_works_hashset_1() {
        assert_eq!(first_marker_hashset(INPUT1, 4), 7);
        assert_eq!(first_marker_hashset(INPUT2, 4), 5);
        assert_eq!(first_marker_hashset(INPUT3, 4), 6);
        assert_eq!(first_marker_hashset(INPUT4, 4), 10);
        assert_eq!(first_marker_hashset(INPUT5, 4), 11);
    }

    #[test]
    fn it_works_hashset_2() {
        assert_eq!(first_marker_hashset(INPUT1, 14), 19);
        assert_eq!(first_marker_hashset(INPUT2, 14), 23);
        assert_eq!(first_marker_hashset(INPUT3, 14), 23);
        assert_eq!(first_marker_hashset(INPUT4, 14), 29);
        assert_eq!(first_marker_hashset(INPUT5, 14), 26);
    }

    #[bench]
    fn bench_first_marker_hashset_04(b: &mut Bencher) {
        b.iter(|| first_marker_hashset(INPUT, 4));
    }

    #[bench]
    fn bench_first_marker_hashset_08(b: &mut Bencher) {
        b.iter(|| first_marker_hashset(INPUT, 8));
    }

    #[bench]
    fn bench_first_marker_hashset_12(b: &mut Bencher) {
        b.iter(|| first_marker_hashset(INPUT, 12));
    }

    #[bench]
    fn bench_first_marker_hashset_14(b: &mut Bencher) {
        b.iter(|| first_marker_hashset(INPUT, 14));
    }

    #[bench]
    fn bench_first_marker_04(b: &mut Bencher) {
        b.iter(|| first_marker(INPUT, 4));
    }

    #[bench]
    fn bench_first_marker_08(b: &mut Bencher) {
        b.iter(|| first_marker(INPUT, 8));
    }

    #[bench]
    fn bench_first_marker_12(b: &mut Bencher) {
        b.iter(|| first_marker(INPUT, 12));
    }

    #[bench]
    fn bench_first_marker_14(b: &mut Bencher) {
        b.iter(|| first_marker(INPUT, 14));
    }
}
