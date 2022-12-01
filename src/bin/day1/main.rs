fn calories<const N: usize>(input: &str) -> u64 {
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

fn main() {
    let input = include_str!("input");
    println!("{}", calories::<2>(input));
    println!("{}", calories::<4>(input));
}

#[cfg(test)]
mod tests {
    use crate::calories;

    const TEST_INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(calories::<2>(TEST_INPUT), 24000);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(calories::<4>(TEST_INPUT), 45000);
    }
}
