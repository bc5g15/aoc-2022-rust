use std::collections::HashSet;

pub fn first_marker(input: &String, length: u32) -> Option<u32> {
    let length = length as usize;
    for i in 0..input.len()-(length-1) {
        let part = &input[i..=i+(length-1)];
        if all_chars_differ(part) {
            return Some((i+length) as u32);
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
        assert_eq!(first_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4),     Some(7));
        assert_eq!(first_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4),       Some(5));
        assert_eq!(first_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),       Some(6));
        assert_eq!(first_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),  Some(10));
        assert_eq!(first_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),   Some(11));
    }

    #[test]
    fn part_two() {
        assert_eq!(first_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),     Some(19));
        assert_eq!(first_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),       Some(23));
        assert_eq!(first_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),       Some(23));
        assert_eq!(first_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),  Some(29));
        assert_eq!(first_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),   Some(26));
    }
}
