use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Elem {
    Int(u64),
    LB,
    RB,
}

struct ElemIter<'a> {
    inner: &'a str,
}

impl<'a> Iterator for ElemIter<'a> {
    type Item = Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.starts_with(',') {
            self.inner = &self.inner[1..];
        }
        if self.inner.starts_with('[') {
            self.inner = &self.inner[1..];
            return Some(Elem::LB);
        }
        if self.inner.starts_with(']') {
            self.inner = &self.inner[1..];
            return Some(Elem::RB);
        }
        let Some(idx) = self.inner.find([',', ']']) else {
            return None
        };
        let (head, tail) = self.inner.split_at(idx);
        self.inner = tail;
        Some(Elem::Int(head.parse::<u64>().unwrap()))
    }
}

fn balance(comp: u64, elems: &mut ElemIter) -> Ordering {
    let mut depth = 1;
    loop {
        match elems.next().unwrap() {
            Elem::LB => depth += 1,
            Elem::RB => return Ordering::Greater,
            Elem::Int(other) => {
                let ord = comp.cmp(&other);
                if !ord.is_eq() {
                    return ord;
                }
                for _ in 0..depth {
                    if elems.next().unwrap() != Elem::RB {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

fn cmp_pair(left: &&str, right: &&str) -> Ordering {
    let mut left_elems = ElemIter { inner: left };
    let mut right_elems = ElemIter { inner: right };
    loop {
        let ord = match (left_elems.next(), right_elems.next()) {
            (Some(Elem::RB), Some(Elem::RB)) => Ordering::Equal,
            (Some(Elem::LB), Some(Elem::LB)) => Ordering::Equal,
            (Some(Elem::RB), Some(Elem::LB | Elem::Int(_))) => Ordering::Less,
            (Some(Elem::LB | Elem::Int(_)), Some(Elem::RB)) => Ordering::Greater,
            (Some(Elem::Int(l)), Some(Elem::Int(r))) => l.cmp(&r),
            (Some(Elem::Int(l)), Some(Elem::LB)) => balance(l, &mut right_elems),
            (Some(Elem::LB), Some(Elem::Int(r))) => balance(r, &mut left_elems).reverse(),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => panic!(),
        };
        if !ord.is_eq() {
            return ord;
        }
    }
}

fn find_ordered(input: &str) -> usize {
    let mut right_order_sum = 0;
    for (idx, pair) in input.split("\n\n").enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        if cmp_pair(&left, &right).is_lt() {
            right_order_sum += idx + 1
        }
    }
    right_order_sum
}

fn order(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .chain(["[[2]]", "[[6]]"])
        .collect::<Vec<_>>();
    packets.sort_unstable_by(cmp_pair);
    let pos1 = packets.iter().position(|&l| l == "[[2]]").unwrap() + 1;
    let pos2 = packets.iter().position(|&l| l == "[[6]]").unwrap() + 1;
    pos1 * pos2
}

fn main() {
    let input = include_str!("input");
    println!("{}", find_ordered(input));
    println!("{}", order(input));
}

#[cfg(test)]
mod tests {
    use crate::{find_ordered, order};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(find_ordered(INPUT), 13);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(order(INPUT), 140);
    }
}
