use std::collections::HashSet;

fn prepare(s: &str) -> (Vec<&[u8]>, Vec<Vec<u8>>) {
    let mut vecs = Vec::<&[u8]>::new();
    for line in s.lines() {
        vecs.push(line.as_bytes());
    }

    let mut vecs_t = vec![Vec::<u8>::new(); vecs[0].len()];
    for vec in vecs.iter() {
        for (i, &v) in vec.iter().enumerate() {
            vecs_t[i].push(v)
        }
    }

    (vecs, vecs_t)
}

fn find_from_front<'a>(
    line: impl Iterator<Item = (usize, &'a u8)>,
    mut insert: impl FnMut(usize) -> bool,
) {
    let mut tallest: Option<u8> = None;
    for (k, &tree) in line {
        if tallest.map(|t| tree > t).unwrap_or(true) {
            tallest = Some(tree);
            insert(k);
        }
    }
}

fn find_visible(s: &str) -> usize {
    let (vecs, vecs_t) = prepare(s);

    let mut set = HashSet::<(usize, usize)>::new();
    for (j, line) in vecs.iter().enumerate() {
        find_from_front(line.iter().enumerate(), |k| set.insert((j, k)));
        find_from_front(line.iter().enumerate().rev(), |k| set.insert((j, k)));
    }
    for (j, line) in vecs_t.iter().enumerate() {
        find_from_front(line.iter().enumerate(), |k| set.insert((k, j)));
        find_from_front(line.iter().enumerate().rev(), |k| set.insert((k, j)));
    }

    set.len()
}

fn find_scenic(s: &str) -> usize {
    let (vecs, vecs_t) = prepare(s);

    let h = vecs.len();
    let w = vecs_t.len();
    let mut max = 0;
    for j in 0..h {
        for k in 0..w {
            let tree = vecs[j][k];
            let up = (0..j).rev().find(|&i| vecs_t[k][i] >= tree).unwrap_or(0);
            let down = (j + 1..h).find(|&i| vecs_t[k][i] >= tree).unwrap_or(h - 1);
            let left = (0..k).rev().find(|&i| vecs[j][i] >= tree).unwrap_or(0);
            let right = (k + 1..w).find(|&i| vecs[j][i] >= tree).unwrap_or(w - 1);
            let m = (j - up) * (down - j) * (k - left) * (right - k);
            if m > max {
                max = m
            }
        }
    }

    max
}

fn main() {
    let input = include_str!("input");
    println!("{}", find_visible(input));
    println!("{}", find_scenic(input));
}

#[cfg(test)]
mod tests {
    use crate::{find_scenic, find_visible};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(find_visible(INPUT), 21);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find_scenic(INPUT), 8);
    }
}
