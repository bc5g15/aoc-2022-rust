use std::cmp::max;

type Coord = (i32, i32);
type SensorBeacon = (Coord, Coord);

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

pub fn retro_part_one(input: &String, target_row: i32) -> i32 {
    let sbs = read_report(input);
    let sensor_radii = calculate_radii(sbs);

    let rs = resolve_ranges_for_row(&sensor_radii, target_row);
    rs.iter().map(|(start, end)| end - start).sum()
}

pub fn find_range_gap(input: &String, square_max: i32) -> i64 {
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

type SensorRadius = (Coord, i32);

fn calculate_radii(sensors: Vec<SensorBeacon>) -> Vec<SensorRadius>{
    sensors.iter().map(|((x, y), (a, b))| {
        let radius: i32 = (x.abs_diff(*a) + y.abs_diff(*b)) as i32;
        return ((*x, *y), radius);
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(retro_part_one(&input, 10), 26);
        assert_eq!(find_range_gap(&input, 20), 56000011);
    }
}
