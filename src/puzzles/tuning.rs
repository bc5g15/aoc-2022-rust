use std::collections::HashSet;

pub fn first_marker(input: &String) -> Option<u32> {
    for i in 0..input.len()-3 {
        let part = &input[i..=i+3];
        if all_chars_differ(part) {
            return Some((i+4) as u32);
        }
    }
    return None;
}

fn all_chars_differ(input: &str) -> bool{
    let char_set: HashSet<char> = HashSet::from_iter(input.chars());
    char_set.len() == input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(first_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),     Some(7));
        assert_eq!(first_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),       Some(5));
        assert_eq!(first_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()),       Some(6));
        assert_eq!(first_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),  Some(10));
        assert_eq!(first_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),   Some(11));

    }
}