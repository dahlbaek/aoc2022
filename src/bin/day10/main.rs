fn tick(x: i64, cycle: &mut i64, acc: &mut i64, crt: &mut String) {
    let pixel_position = *cycle % 40;
    *cycle += 1;

    if pixel_position >= x - 1 && pixel_position <= x + 1 {
        crt.push('#')
    } else {
        crt.push('.')
    }

    if pixel_position == 19 {
        *acc += *cycle * x;
    } else if pixel_position == 39 {
        crt.push('\n')
    }
}

fn run_instructions(s: &str) -> (i64, String) {
    let mut cycle = 0;
    let mut x = 1;
    let mut acc_signal_strength = 0;
    let mut crt = String::new();

    for instruction in s.lines() {
        if instruction.starts_with('n') {
            tick(x, &mut cycle, &mut acc_signal_strength, &mut crt);
        } else {
            tick(x, &mut cycle, &mut acc_signal_strength, &mut crt);
            tick(x, &mut cycle, &mut acc_signal_strength, &mut crt);
            x += instruction[5..].parse::<i64>().unwrap();
        }
    }

    (acc_signal_strength, crt)
}

fn main() {
    let input = include_str!("input");
    let (signal_strength_acc, display) = run_instructions(input);
    println!("{}", signal_strength_acc);
    println!("{}", display);
}

#[cfg(test)]
mod tests {
    use crate::run_instructions;

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(run_instructions(INPUT).0, 13140);
    }

    const DISPLAY: &'static str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    #[test]
    fn it_works_2() {
        assert_eq!(run_instructions(INPUT).1, DISPLAY);
    }
}
