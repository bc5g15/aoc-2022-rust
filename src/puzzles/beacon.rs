use std::collections::HashSet;

type Coord = (i32, i32);
type SensorBeacon = (Coord, Coord);

type VisitSpace = HashSet<Coord>;

fn read_report(input: &String) -> Vec<SensorBeacon> {
    let quick_parse = |s: &str| -> i32 {s.chars().filter(|p| p.is_digit(10) || p == &'-').collect::<String>().parse().unwrap()};

    input.trim().lines().map(|line| {
        let words: Vec<&str> = line.trim().split(" ").collect();
        let x1 = quick_parse(words[2]);
        let y1 = quick_parse(words[3]);
        let x2 = quick_parse(words[8]);
        let y2 = quick_parse(words[9]);
        ((x1, y1), (x2, y2))
    }).collect()
}

fn all_checked_spaces(sb: SensorBeacon) -> VisitSpace {
    let ((x, y), (a, b)) = sb;

    let radius: i32  = (x.abs_diff(a) + y.abs_diff(b)) as i32;
    let mut visited: VisitSpace = HashSet::new();

    for i in (-radius)..=radius {
        let thickness: i32 = i.abs().abs_diff(radius) as i32;
        for j in (x-thickness)..=(x+thickness) {
            visited.insert((j, y+i));
        }
    }
    visited
}

pub fn all_seen_at_row(input: &String, row: i32) -> usize {
    let sbs = read_report(input);
    let mut occupied_spaces: VisitSpace = HashSet::new();
    for (s, b) in sbs.iter() {
        occupied_spaces.insert(*s);
        occupied_spaces.insert(*b);
    }

    let all_seen = read_report(input).iter().map(|sb| all_checked_spaces(*sb))
        .reduce(|acc, cur| acc.union(&cur).map(|c| *c).collect()).unwrap();
    
    all_seen.difference(&occupied_spaces).filter(|(_x, y)| *y==row).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn read_neg() {
        let trouble = ((0,11), (2,10));
        let mut spaces: Vec<Coord> = all_checked_spaces(trouble).iter().map(|c| *c).collect();
        spaces.sort_by_key(|(_x, y)| *y);
        dbg!(spaces);
    }

    #[test]
    fn example_one() {
        let input = r"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        ".to_string();

        assert_eq!(all_seen_at_row(&input, 10), 26);
    }
}