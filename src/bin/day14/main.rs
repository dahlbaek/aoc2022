fn prepare(line: &str, recenter: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
    line.split(" -> ").map(move |t| {
        let (x, y) = t.split_once(',').unwrap();
        let x_recentered = x.parse::<isize>().unwrap() - recenter;
        (x_recentered, y.parse::<isize>().unwrap())
    })
}

fn find_boundaries(input: &str, open: bool) -> (isize, usize, usize) {
    let (mut x_min, mut x_max, mut y_max) = (0, 0, 0);
    for line in input.lines() {
        for point in prepare(line, 500) {
            x_min = x_min.min(point.0);
            x_max = x_max.max(point.0);
            y_max = y_max.max(point.1);
        }
    }

    if open {
        (
            -x_min,
            usize::try_from(x_max - x_min + 1).unwrap(),
            usize::try_from(y_max + 1).unwrap(),
        )
    } else {
        let y_max_res = y_max + 2;
        (
            -x_min.min(-y_max_res),
            usize::try_from(x_max.max(y_max_res) - x_min.min(-y_max_res) + 1).unwrap(),
            usize::try_from(y_max_res + 1).unwrap(),
        )
    }
}

fn parse_scan(input: &str, open: bool) -> (Vec<bool>, usize, usize) {
    let (starting_position, row_len, col_len) = find_boundaries(input, open);

    let mut scan = vec![false; row_len * col_len];
    for line in input.lines() {
        let mut paths = prepare(line, 500 - starting_position)
            .map(|(a, b)| (usize::try_from(a).unwrap(), usize::try_from(b).unwrap()));

        let mut path_start = paths.next().unwrap();
        for path_end in paths {
            for x in (path_start.0..=path_end.0).chain(path_end.0..=path_start.0) {
                scan[x + path_start.1 * row_len] = true;
            }
            for y in (path_start.1..=path_end.1).chain(path_end.1..=path_start.1) {
                scan[path_start.0 + y * row_len] = true;
            }
            path_start = path_end;
        }
    }
    if !open {
        for entry in scan.iter_mut().rev().take(row_len) {
            *entry = true;
        }
    }

    (scan, starting_position.try_into().unwrap(), row_len)
}

enum Action {
    Down,
    Left,
    Right,
    Fall,
    Stay,
}

fn action(current_idx: usize, row_len: usize, scan: &[bool]) -> Action {
    if current_idx >= scan.len() - row_len {
        Action::Fall
    } else if !scan[current_idx + row_len] {
        Action::Down
    } else if current_idx % row_len == 0 {
        Action::Fall
    } else if !scan[current_idx + row_len - 1] {
        Action::Left
    } else if (current_idx + 1) % row_len == 0 {
        Action::Fall
    } else if !scan[current_idx + row_len + 1] {
        Action::Right
    } else {
        Action::Stay
    }
}

fn find(input: &str, open: bool) -> usize {
    let (mut scan, starting_position, row_len) = parse_scan(input, open);
    let mut sand = 0;
    let mut current_idx = starting_position;
    loop {
        match action(current_idx, row_len, &scan) {
            Action::Down => current_idx += row_len,
            Action::Left => current_idx += row_len - 1,
            Action::Right => current_idx += row_len + 1,
            Action::Fall => break,
            Action::Stay => {
                scan[current_idx] = true;
                sand += 1;
                if current_idx == starting_position {
                    break;
                }
                current_idx = starting_position;
            }
        }
    }
    return sand;
}

fn main() {
    let input = include_str!("input");
    println!("{}", find(input, true));
    println!("{}", find(input, false));
}

#[cfg(test)]
mod tests {
    use crate::find;

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(find(INPUT, true), 24);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find(INPUT, false), 93);
    }
}
