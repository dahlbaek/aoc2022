fn find_top(l: &str, reverse: bool) -> String {
    let (start_state, instructions) = l.split_once("\n\n").unwrap();
    let mut lines = start_state.lines().rev();
    let num_vecs = 1 + (lines.next().unwrap().as_bytes().len() - 3) / 4;
    let mut vecs = vec![<Vec<u8>>::new(); num_vecs];
    for line in lines {
        let bytes = line.as_bytes();
        for (idx, vec) in vecs.iter_mut().enumerate() {
            let byte = bytes[1 + idx * 4];
            if byte != b' ' {
                vec.push(byte);
            }
        }
    }

    for instruction in instructions.lines() {
        let mut syms = instruction.split_ascii_whitespace();
        let num = syms.nth(1).unwrap().parse::<usize>().unwrap();
        let from = syms.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = syms.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let popped = vecs[from].pop().unwrap();
            vecs[to].push(popped);
        }
        if reverse {
            let last = vecs[to].len() - num;
            vecs[to][last..].reverse();
        }
    }

    let mut s = String::new();
    for mut vec in vecs {
        s.push(char::try_from(vec.pop().unwrap()).unwrap())
    }
    s
}

fn main() {
    let input = include_str!("input");
    println!("{}", &find_top(input, false));
    println!("{}", &find_top(input, true));
}

#[cfg(test)]
mod tests {
    use crate::find_top;

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(&find_top(INPUT, false), "CMZ");
    }

    #[test]
    fn it_works_2() {
        assert_eq!(&find_top(INPUT, true), "MCD");
    }
}
