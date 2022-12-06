use std::collections::HashSet;

fn first_marker(l: &str, size: usize) -> usize {
    size + l
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, bytes)| HashSet::<&u8>::from_iter(bytes.iter()).len() == size)
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

    const INPUT1: &'static str = include_str!("test-input-1");
    const INPUT2: &'static str = include_str!("test-input-2");
    const INPUT3: &'static str = include_str!("test-input-3");
    const INPUT4: &'static str = include_str!("test-input-4");
    const INPUT5: &'static str = include_str!("test-input-5");

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
}
