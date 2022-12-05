#![feature(test)]

use std::{cmp::Reverse, collections::BinaryHeap};

extern crate test;

fn calories(input: &str, capacity: usize) -> u64 {
    let mut heap = BinaryHeap::<Reverse<u64>>::with_capacity(capacity);
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            heap.push(Reverse(current));
            if heap.len() >= capacity {
                heap.pop().unwrap();
            }
            current = 0;
            continue;
        }
        current += line.parse::<u64>().unwrap()
    }
    heap.push(Reverse(current));
    if heap.len() >= capacity {
        heap.pop().unwrap();
    }
    heap.into_iter().map(|r| r.0).sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", calories(input, 2));
    println!("{}", calories(input, 4));
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::calories;

    const TEST_INPUT: &'static str = include_str!("test-input");
    const INPUT: &'static str = include_str!("foo.txt");

    fn vec_sort_calories<const N: usize>(input: &str) -> u64 {
        let mut acc = [0u64; N];
        for line in input.lines() {
            if line.is_empty() {
                acc.sort_unstable();
                acc[0] = 0;
                continue;
            }
            acc[0] += line.parse::<u64>().unwrap()
        }
        acc.sort_unstable();
        acc[1..N].into_iter().sum()
    }

    fn manual_sort_calories<const N: usize>(input: &str) -> u64 {
        let mut acc = [0u64; N];
        for line in input.lines() {
            if line.is_empty() {
                let new_value = acc[0];
                let partition_point = acc.partition_point(|&i| i <= new_value);
                acc[..partition_point].rotate_left(1);
                acc[0] = 0;
                continue;
            }
            acc[0] += line.parse::<u64>().unwrap()
        }
        let new_value = acc[0];
        let partition_point = acc[1..].partition_point(|&i| i <= new_value);
        acc[partition_point] = 0;
        acc.into_iter().sum()
    }

    fn track_index_calories<const N: usize>(input: &str) -> u64 {
        let mut acc = [0u64; N];
        let mut idx = 0;
        for line in input.lines() {
            if line.is_empty() {
                idx = acc.iter().enumerate().min_by_key(|(_, &i)| i).unwrap().0;
                acc[idx] = 0;
                continue;
            }
            acc[idx] += line.parse::<u64>().unwrap()
        }
        idx = acc.iter().enumerate().min_by_key(|(_, &i)| i).unwrap().0;
        acc[idx] = 0;
        acc.into_iter().sum()
    }

    #[test]
    fn it_works_1() {
        assert_eq!(calories(TEST_INPUT, 2), 24000);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(calories(TEST_INPUT, 4), 45000);
    }

    #[test]
    fn vec_sort_works_1() {
        assert_eq!(vec_sort_calories::<2>(TEST_INPUT), 24000);
    }

    #[test]
    fn vec_sort_works_2() {
        assert_eq!(vec_sort_calories::<4>(TEST_INPUT), 45000);
    }

    #[test]
    fn manual_sort_works_1() {
        assert_eq!(manual_sort_calories::<2>(TEST_INPUT), 24000);
    }

    #[test]
    fn manual_sort_works_2() {
        assert_eq!(manual_sort_calories::<4>(TEST_INPUT), 45000);
    }

    #[test]
    fn track_index_calories_works_1() {
        assert_eq!(track_index_calories::<2>(TEST_INPUT), 24000);
    }

    #[test]
    fn track_index_calories_works_2() {
        assert_eq!(track_index_calories::<4>(TEST_INPUT), 45000);
    }

    #[bench]
    fn bench_calories_0004(b: &mut Bencher) {
        b.iter(|| calories(INPUT, 4));
    }

    #[bench]
    fn bench_calories_0100(b: &mut Bencher) {
        b.iter(|| calories(INPUT, 100));
    }

    #[bench]
    fn bench_calories_1000(b: &mut Bencher) {
        b.iter(|| calories(INPUT, 1000));
    }

    #[bench]
    fn bench_calories_5000(b: &mut Bencher) {
        b.iter(|| calories(INPUT, 5000));
    }

    #[bench]
    fn bench_vec_sort_calories_0004(b: &mut Bencher) {
        b.iter(|| vec_sort_calories::<4>(INPUT));
    }

    #[bench]
    fn bench_vec_sort_calories_0100(b: &mut Bencher) {
        b.iter(|| vec_sort_calories::<100>(INPUT));
    }

    #[bench]
    fn bench_vec_sort_calories_1000(b: &mut Bencher) {
        b.iter(|| vec_sort_calories::<1000>(INPUT));
    }

    #[bench]
    fn bench_vec_sort_calories_5000(b: &mut Bencher) {
        b.iter(|| vec_sort_calories::<5000>(INPUT));
    }

    #[bench]
    fn bench_manual_sort_calories_0004(b: &mut Bencher) {
        b.iter(|| manual_sort_calories::<4>(INPUT));
    }

    #[bench]
    fn bench_manual_sort_calories_0100(b: &mut Bencher) {
        b.iter(|| manual_sort_calories::<100>(INPUT));
    }

    #[bench]
    fn bench_manual_sort_calories_1000(b: &mut Bencher) {
        b.iter(|| manual_sort_calories::<1000>(INPUT));
    }

    #[bench]
    fn bench_manual_sort_calories_5000(b: &mut Bencher) {
        b.iter(|| manual_sort_calories::<5000>(INPUT));
    }

    #[bench]
    fn bench_track_index_calories_0004(b: &mut Bencher) {
        b.iter(|| track_index_calories::<4>(INPUT));
    }

    #[bench]
    fn bench_track_index_calories_0100(b: &mut Bencher) {
        b.iter(|| track_index_calories::<100>(INPUT));
    }

    #[bench]
    fn bench_track_index_calories_1000(b: &mut Bencher) {
        b.iter(|| track_index_calories::<1000>(INPUT));
    }

    #[bench]
    fn bench_track_index_calories_5000(b: &mut Bencher) {
        b.iter(|| track_index_calories::<5000>(INPUT));
    }
}
