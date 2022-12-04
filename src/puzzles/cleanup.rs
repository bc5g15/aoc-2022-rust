
type Range = (u32, u32);
type RangePair = (Range, Range);

fn read_range(input: &str) -> Range {
    let numbers: Vec<u32> = input.split("-").map(|p| p.parse().expect("Must be a number")).collect();
    let one = numbers.get(0).expect("Must have a first number");
    let two = numbers.get(1).expect("Must have a second number");
    (*one, *two)
}

fn read_pairs(input: &str) -> RangePair {
    let parts: Vec<Range> = input.split(",").map(|s| read_range(s)).collect();
    let one = parts.get(0).expect("Must have a first range");
    let two = parts.get(1).expect("Must have a second range");
    (*one, *two)
}

fn has_containment(pair: RangePair) -> bool{
    let ((a, b), (x, y)) = pair;
    if a<=x && b>=y {
        return true;
    }
    if x<=a && y>=b {
        return true;
    }
    false
}

pub fn how_many_containments(input: &String) -> u32 {
    input.trim().lines()
        .map(|l| read_pairs(l))
        .filter(|rp| has_containment(*rp))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        ".to_string();

        let count = how_many_containments(&input);

        assert_eq!(count, 2);
    }
}