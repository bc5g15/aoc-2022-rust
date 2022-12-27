use std::collections::HashSet;
use std::cmp::max;

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

fn all_checked_spaces(sb: SensorBeacon, row: i32) -> VisitSpace {
    let ((x, y), (a, b)) = sb;

    let radius: i32  = (x.abs_diff(a) + y.abs_diff(b)) as i32;
    let mut visited: VisitSpace = HashSet::new();

    for i in (-radius)..=radius {
        if (y+i) == row {
            let thickness: i32 = i.abs().abs_diff(radius) as i32;
            for j in (x-thickness)..=(x+thickness) {
                visited.insert((j, y+i));
            }
        }
    }
    visited
}

type View = (i32, i32);

fn range_for_row(sr: SensorRadius, row: i32) -> Option<View> {
    let ((x, y), r) = sr;

    let relative_row = row - y; 

    if relative_row.abs() > r {
        return None;
    }

    let thickness = relative_row.abs().abs_diff(r) as i32;
    return Some((x-thickness, x+thickness));
}

fn resolve_ranges_for_row(srs: &Vec<SensorRadius>, row: i32) -> Vec<View> {
    let mut views: Vec<View> = srs.iter().map(|sr| range_for_row(*sr, row)).filter(|r| r.is_some())
        .map(|r| r.unwrap()).collect();
    views.sort();

    let mut merged: Vec<View> = Vec::new();
    // dbg!(row, &views);

    let (mut start, mut end) = views[0];

    for v in views.iter().skip(1) {
        let (new_start, new_end) = *v;
        if new_start-1 <= end {
            end = max(new_end, end);
        } else {
            merged.push((start, end));
            start = new_start;
            end = new_end;
        }
    }
    merged.push((start, end));
    merged
}

pub fn find_range_gap(input: &String,square_max: i32) -> i64 {
    let sbs = read_report(input);
    let sensor_radii = calculate_radii(sbs);

    for i in 0..=square_max {
        let k = resolve_ranges_for_row(&sensor_radii, i);
        if k.len() == 2 {
            let row = i;
            let (_x, y) = k[0];
            let col = y + 1;
            return (col as i64) * 4000000 + row as i64;
        }
    }
    panic!("Nope!");
}

pub fn all_seen_at_row(input: &String, row: i32) -> usize {
    let sbs = read_report(input);
    let mut occupied_spaces: VisitSpace = HashSet::new();
    for (s, b) in sbs.iter() {
        occupied_spaces.insert(*s);
        occupied_spaces.insert(*b);
    }

    let all_seen = read_report(input).iter().map(|sb| all_checked_spaces(*sb, row))
        .reduce(|acc, cur| acc.union(&cur).map(|c| *c).collect()).unwrap();
    
    all_seen.difference(&occupied_spaces).filter(|(_x, y)| *y==row).count()
}

type SensorRadius = (Coord, i32);

fn calculate_radii(sensors: Vec<SensorBeacon>) -> Vec<SensorRadius>{
    sensors.iter().map(|((x, y), (a, b))| {
        let radius: i32 = (x.abs_diff(*a) + y.abs_diff(*b)) as i32;
        return ((*x, *y), radius);
    }).collect()
}

fn check_containment(sr: SensorRadius, location: Coord) -> bool {
    let ((x, y), r) = sr;
    let (a, b) = location;

    let new_r = (x.abs_diff(a) + y.abs_diff(b)) as i32;
    new_r <= r
}

pub fn find_absent_position_in_square(input: &String, square_max: i32) -> Coord {
    let sbs = read_report(input);
    let sensor_radii = calculate_radii(sbs);

    for i in 0..=square_max {
        for j in 0..=square_max {
            if sensor_radii.iter().all(|s| !check_containment(*s, (i, j))) {
                return (i, j)
            }
        }
    }
    panic!("No luck!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn read_neg() {
        let trouble = ((0,11), (2,10));
        let mut spaces: Vec<Coord> = all_checked_spaces(trouble, 10).iter().map(|c| *c).collect();
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
        assert_eq!(find_absent_position_in_square(&input, 20), (14, 11));
        assert_eq!(find_range_gap(&input, 20), 56000011);
    }
}
