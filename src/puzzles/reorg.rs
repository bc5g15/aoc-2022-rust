use std::collections::HashSet;

fn pack_input(input: &String) -> (String, String) {
    let midpoint = input.len() / 2;
    let (lhs, rhs) = input.split_at(midpoint);
    (lhs.to_string(), rhs.to_string())
}

fn find_shared(left: &String, right: &String) -> Vec<u8>{
    let left_set: HashSet<u8> = HashSet::from_iter(left.bytes());
    let right_set: HashSet<u8> = HashSet::from_iter(right.bytes());

    left_set.intersection(&right_set).map(|c| *c).collect()
}

fn get_value(c: u8) -> u32 {
    if c <= b'Z' {
        ((c - b'A') +27) as u32
    } else {
        ((c - b'a') + 1) as u32
    }
}

pub fn value_shared_priorities(input: &String) -> u32{
    input.trim().lines().fold(0, |acc, line| {
        let (lhs, rhs) = pack_input(&line.to_string());
        let shared = find_shared(&lhs, &rhs);
        acc + shared.iter().fold(0, |acc, byte| get_value(*byte) + acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let midpoint = input.len() /2;
        let (lhs, rhs) = input.split_at(midpoint);
        assert_eq!(lhs, "vJrwpWtwJgWr");
        assert_eq!(rhs, "hcsFMMfFFhFp");
    }

    #[test]
    fn byte_test() {
        let input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let bytes: Vec<u8> = input.bytes().map(|b| b).collect();
        dbg!(bytes);
    }

    #[test]
    fn set_test() {
        let input = "abcabc";
        let collection: HashSet<char> = HashSet::from_iter(input.chars());
        dbg!(collection);
    }

    #[test]
    fn check_shared() {
        let left = "vJrwpWtwJgWr".to_string();
        let right = "hcsFMMfFFhFp".to_string();

        let shared = find_shared(&left, &right);
        dbg!(shared);
    }

    #[test]
    fn part_one_example() {
        let input = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        ".to_string();

        assert_eq!(value_shared_priorities(&input), 157);
    }
}