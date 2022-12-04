fn read_field<const C: char>(l: &str) -> (u64, &str) {
    let (res, s) = l.split_once(C).unwrap();
    (res.parse::<u64>().unwrap(), s)
}

fn prepare(l: &str) -> (u64, u64, u64, u64) {
    let (a_1, l) = read_field::<'-'>(l);
    let (a_2, l) = read_field::<','>(l);
    let (b_1, l) = read_field::<'-'>(l);
    (a_1, a_2, b_1, l.parse::<u64>().unwrap())
}

fn contains((a1, a2, b1, b2): (u64, u64, u64, u64)) -> u64 {
    u64::from(a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2)
}

fn overlaps((a1, a2, b1, b2): (u64, u64, u64, u64)) -> u64 {
    u64::from(!(a2 < b1 || b2 < a1))
}

fn main() {
    let input = include_str!("input");
    println!("{}", input.lines().map(prepare).map(contains).sum::<u64>());
    println!("{}", input.lines().map(prepare).map(overlaps).sum::<u64>());
}

#[cfg(test)]
mod tests {
    use crate::{contains, overlaps, prepare};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(INPUT.lines().map(prepare).map(contains).sum::<u64>(), 2);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(INPUT.lines().map(prepare).map(overlaps).sum::<u64>(), 4);
    }
}
