use std::str::Chars;
use std::cmp::{max, Ordering};

fn read_pairs(input: &String) -> Vec<(RecurPacket, RecurPacket)>{
    let vectored: Vec<&str> = input.trim().lines().collect();
    let grouped: Vec<&[&str]> = vectored.split(|s| *s == "").collect();

    grouped.iter().map(|group| {
        let first = group[0];
        let second = group[1];
        (recur_parse_root(first), recur_parse_root(second))
    }).collect()
}

#[derive(Debug, Clone)]
enum RecurPacket {
    Value(u32),
    Arr (Vec<RecurPacket>)
}

fn recur_parse_root(input: &str) -> RecurPacket {
    let mut iter = input.chars();
    match recur_parse(&mut iter) {
        RecurPacket::Arr(ar) => {
            ar[0].to_owned()
        },
        n => n
    }
}

fn recur_parse(current_text: &mut Chars) -> RecurPacket{
    use RecurPacket::*;

    let mut value_arr: Vec<RecurPacket> = Vec::new();

    while let Some(c) = current_text.next() {
        if c.is_digit(10) {
            let mut number = vec![c];
            current_text.by_ref().take_while(|n| n.is_digit(10))
                .for_each(|n| number.push(n));
            value_arr.push(Value (number.iter().collect::<String>().parse().unwrap()));
            continue;
        }

        match c {
            '[' => value_arr.push(recur_parse(current_text)),
            ']' => return Arr (value_arr),
            ',' => continue,
            _ => panic!("Unrecognised entry {c}")
        }
    }
    Arr (value_arr)
}

fn compare(a: RecurPacket, b: RecurPacket) -> Ordering{
    use RecurPacket::*;
    use Ordering::*;
    match (a, b) {
        (Value(av), Value(bv)) => {
            return av.cmp(&bv);
        },
        (Arr(ar), Arr(br)) => {
            for i in 0..max(ar.len(), br.len()) {
                let ag = ar.get(i);
                let bg = br.get(i);

                match (ag, bg) {
                    (None, None) => {
                        // Can't happen, but need to handle it
                        return Less;
                    },
                    (Some(_), None) => {
                        return Greater;
                    },
                    (None, Some(_)) => {
                        return Less;
                    },
                    (Some(ax), Some(bx)) => {
                        match compare(ax.to_owned(), bx.to_owned()) {
                            Greater => return Greater,
                            Less => return Less,
                            Equal => continue
                        }
                    }
                }
            }
            // If equal and of equal length, equal, yeah
            return Equal
        },
        (Value(av), Arr(br)) => {
            return compare(Arr(vec![Value(av)]), Arr(br));
        },
        (Arr(ar), Value(bv)) => {
            return compare(Arr(ar), Arr(vec![Value(bv)]));
        }
    }
}

pub fn evaluate_sorted(input: &String) -> usize {
    let pairs = read_pairs(input);
    pairs.iter().enumerate()
        .filter(|(_, (a, b))| compare(a.to_owned(), b.to_owned()) != Ordering::Greater)
        .fold(0, |acc, (v,(_, _))| (v+1)+acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_tests() {
        dbg!(recur_parse_root("[[[9]]]"));
    }

    #[test]
    fn comparison_test() {
        let input = r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
        ".to_string();

        assert_eq!(evaluate_sorted(&input), 13);
    }
}