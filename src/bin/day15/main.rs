use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug)]
struct Line {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl Line {
    fn get_coverage_at(&self, y: i64) -> (i64, i64) {
        //impl Iterator<Item = i64> {
        let max_coverage = {
            let x_dist = (self.sensor.0 - self.beacon.0).abs();
            let y_dist = (self.sensor.1 - self.beacon.1).abs();
            x_dist + y_dist
        };
        let coverage = max_coverage - (self.sensor.1 - y).abs();
        (self.sensor.0 - coverage, self.sensor.0 + coverage)
    }
}

fn parse_line(mut line: &str) -> Line {
    line = &line[12..];
    let (sensor_x, mut line) = line.split_once(',').unwrap();
    line = &line[3..];
    let (sensor_y, mut line) = line.split_once(':').unwrap();
    line = &line[24..];
    let (beacon_x, line) = line.split_once(',').unwrap();
    let beacon_y = &line[3..];
    Line {
        sensor: (sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
        beacon: (beacon_x.parse().unwrap(), beacon_y.parse().unwrap()),
    }
}

fn find(input: &str, y: i64) -> usize {
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();
    let known_beacons = lines
        .iter()
        .map(|l| l.beacon)
        .filter(|p| p.1 == y)
        .map(|p| p.0)
        .collect::<HashSet<_>>();
    lines
        .iter()
        .flat_map(|l| {
            let (start, end) = l.get_coverage_at(y);
            RangeInclusive::new(start, end)
        })
        .filter(|&c| !known_beacons.contains(&c))
        .collect::<HashSet<_>>()
        .len()
}

fn find2(input: &str, y_range: RangeInclusive<i64>, x_top: i64) -> i64 {
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();
    for y in y_range.clone() {
        let mut excludeds = lines
            .iter()
            .map(|l| l.get_coverage_at(y))
            .filter(|cov| cov.0 <= cov.1)
            .collect::<Vec<_>>();
        excludeds.sort_unstable();
        let mut top = 0;
        for excluded in excludeds {
            if excluded.0 > top + 1 {
                break;
            }
            top = top.max(excluded.1);
        }
        if top < x_top {
            return (top + 1) * 4000000 + y;
        }
    }
    panic!()
}

fn main() {
    let input = include_str!("input");
    println!("{}", find(input, 2000000));
    println!("{}", find2(input, 0..=4000000, 4000000));
}

#[cfg(test)]
mod tests {
    use crate::{find, find2};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1_9() {
        assert_eq!(find(INPUT, 9), 25);
    }

    #[test]
    fn it_works_1_10() {
        assert_eq!(find(INPUT, 10), 26);
    }

    #[test]
    fn it_works_1_11() {
        assert_eq!(find(INPUT, 11), 28);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(find2(INPUT, 0..=20, 20), 56000011);
    }
}
