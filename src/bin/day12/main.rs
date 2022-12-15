#![feature(test)]

extern crate test;

use std::collections::VecDeque;

fn prepare(input: &str) -> (usize, usize, usize, Vec<u8>) {
    let row_len = input.split_once('\n').unwrap().0.len();
    let mut heights = Vec::new();
    let mut start = None;
    let mut end = None;
    for (idx, b) in input.bytes().filter(|&b| b != b'\n').enumerate() {
        match b {
            b'a'..=b'z' => heights.push(b - b'a'),
            b'S' => {
                start = Some(idx);
                heights.push(0)
            }
            b'E' => {
                end = Some(idx);
                heights.push(b'z' - b'a')
            }
            _ => panic!("Unrecognized byte {}", b),
        }
    }
    (start.unwrap(), end.unwrap(), row_len, heights)
}

fn next_positions(pos: usize, row_len: usize, full_len: usize) -> impl Iterator<Item = usize> {
    let mut arr = [0; 4];
    let mut len = 0;
    arr[len] = pos.saturating_add_signed(-1);
    len += (pos % row_len != 0) as usize;
    arr[len] = pos + 1;
    len += ((pos + 1) % row_len != 0) as usize;
    arr[len] = pos.saturating_add_signed(-isize::try_from(row_len).unwrap());
    len += (pos >= row_len) as usize;
    arr[len] = pos + row_len;
    len += (pos < full_len - row_len) as usize;
    arr.into_iter().take(len)
}

fn find(input: &str, find_start: bool) -> usize {
    let (start, end, row_len, heights) = prepare(input);
    let mut explored = vec![None; heights.len()];
    explored[end] = Some(0);
    let mut active = VecDeque::new();
    active.push_back(end);

    loop {
        let current_idx = active.pop_front().unwrap();
        let current_steps = explored[current_idx].unwrap();
        let new_positions = next_positions(current_idx, row_len, heights.len())
            .filter(|&new_idx| heights[new_idx] + 1 >= heights[current_idx]);
        for position in new_positions {
            if explored[position].is_some() {
                continue;
            }
            if (position == start) || (find_start && heights[position] == 0) {
                return current_steps + 1;
            }
            active.push_back(position);
            explored[position] = Some(current_steps + 1);
        }
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", find(input, false));
    println!("{}", find(input, true));
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use test::Bencher;

    use crate::find;

    const TINPUT: &'static str = include_str!("test-input");
    const INPUT: &'static str = include_str!("input");

    fn prepare(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<u8>>) {
        let heights = input
            .lines()
            .map(str::as_bytes)
            .map(|b_arr| {
                b_arr
                    .into_iter()
                    .map(|b| match b {
                        b'a'..=b'z' => b - b'a',
                        b'S' => 0,
                        b'E' => b'z' - b'a',
                        _ => panic!("Unrecognized byte {}", b),
                    })
                    .collect()
            })
            .collect();

        let bytes_idx = input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| l.bytes().enumerate().map(move |(j, b)| ((i, j), b)));
        let (mut start, mut end) = ((0, 0), (0, 0));
        for (idx, b) in bytes_idx {
            if b == b'S' {
                start = idx;
            }
            if b == b'E' {
                end = idx;
            }
        }

        (start, end, heights)
    }

    fn next_positions(idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            (idx.0.saturating_add_signed(-1), idx.1),
            (idx.0.saturating_add_signed(1), idx.1),
            (idx.0, idx.1.saturating_add_signed(-1)),
            (idx.0, idx.1.saturating_add_signed(1)),
        ]
        .into_iter()
        .filter(move |&new_idx| new_idx != idx)
    }

    fn find_with_nested_vecs_and_hashmap(input: &str, find_start: bool) -> usize {
        let (start, end, heights) = prepare(input);
        let mut explored = HashMap::new();
        explored.insert(end, 0);
        let mut active = VecDeque::new();
        active.push_back(end);

        loop {
            let current = active.pop_front().unwrap();
            let current_steps = *explored.get(&current).unwrap();
            let new_positions =
                next_positions(current).filter(|new_idx: &(usize, usize)| -> bool {
                    heights
                        .get(new_idx.0)
                        .and_then(|v| v.get(new_idx.1))
                        .map(|&h| h + 1 >= heights[current.0][current.1])
                        .unwrap_or(false)
                });
            for position in new_positions {
                if explored.contains_key(&position) {
                    continue;
                }
                if (position == start) || (find_start && heights[position.0][position.1] == 0) {
                    return current_steps + 1;
                }
                active.push_back(position);
                explored.insert(position, current_steps + 1);
            }
        }
    }

    #[test]
    fn it_works_1() {
        assert_eq!(find(TINPUT, false), 31);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find(TINPUT, true), 29);
    }

    #[test]
    fn it_works_with_nested_vecs_and_hashmap_1() {
        assert_eq!(find_with_nested_vecs_and_hashmap(TINPUT, false), 31);
    }

    #[test]
    fn it_works_with_nested_vecs_and_hashmap_2() {
        assert_eq!(find_with_nested_vecs_and_hashmap(TINPUT, true), 29);
    }

    #[bench]
    fn bench_find_1(b: &mut Bencher) {
        b.iter(|| find(INPUT, false));
    }

    #[bench]
    fn bench_find_2(b: &mut Bencher) {
        b.iter(|| find(INPUT, true));
    }

    #[bench]
    fn bench_find_with_nested_vecs_and_hashmap_1(b: &mut Bencher) {
        b.iter(|| find_with_nested_vecs_and_hashmap(INPUT, false));
    }

    #[bench]
    fn bench_find_with_nested_vecs_and_hashmap_2(b: &mut Bencher) {
        b.iter(|| find_with_nested_vecs_and_hashmap(INPUT, true));
    }
}
