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

pub fn badge_groups(input: &String) -> u32{
    let mut lines = input.trim().lines().peekable();

    let mut final_value = 0;

    while lines.peek().is_some() { 
        let initial = lines.next().expect("Should be a line here");
        let mut group_shared: HashSet<u8> = HashSet::from_iter(initial.bytes());
        for _ in 1..3 {
            let current = lines.next().expect("Line must exist at this point");
            group_shared = group_shared.intersection(&HashSet::from_iter(current.bytes())).map(|c| *c).collect();
        }

        if group_shared.len() != 1 {
            panic!("Should only be one element left in {group_shared:?}");
        }

        final_value = final_value + group_shared.iter().fold(0, |acc, x| get_value(*x) + acc);
    }

    final_value
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn part_two_example() {
        let input = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        ".to_string();

        assert_eq!(badge_groups(&input), 70);
    }
}