use std::str::Chars;

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

#[derive(Debug)]
enum Packet {
    StepIn,
    Value (u32),
    StepOut
}

fn parse_packet(input: &str) -> Vec<Packet> {
    use Packet::*;
    let mut iter = input.chars();

    let mut output: Vec<Packet> = Vec::new();

    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            let mut number = vec![c];
            iter.by_ref().take_while(|n| n.is_digit(10))
                .for_each(|n| number.push(n));
            output.push(Value (number.iter().collect::<String>().parse().unwrap()));
            continue;
        }

        match c {
            '[' => output.push(StepIn),
            ']' => output.push(StepOut),
            ',' => continue,
            _ => panic!("Unrecognised entry {c}")
        }
    }

    output
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

#[derive(Debug, PartialEq)]
enum Compare {
    Ordered,
    Unordered,
    Equal
}

fn check(a: u32, b: u32) -> Compare {
    use Compare::*;

    if a == b {
        Equal
    } else if a > b {
        Unordered
    } else {
        Ordered
    }
}

fn compare(a: RecurPacket, b: RecurPacket) -> Compare{
    use RecurPacket::*;
    use Compare::*;
    match (a, b) {
        (Value(av), Value(bv)) => {
            return check(av, bv);
        },
        (Arr(ar), Arr(br)) => {
            for i in 0..ar.len() {
                let ag = ar.get(i);
                let bg = br.get(i);

                match (ag, bg) {
                    (None, None) => {
                        return Ordered;
                    },
                    (Some(_), None) => {
                        return Unordered;
                    },
                    (None, Some(_)) => {
                        return Unordered;
                    },
                    (Some(ax), Some(bx)) => {
                        match compare(ax.to_owned(), bx.to_owned()) {
                            Ordered => return Ordered,
                            Unordered => return Unordered,
                            Equal => continue
                        }
                    }
                }
            }
        },
        (Value(av), Arr(br)) => {
            return compare(Arr(vec![Value(av)]), Arr(br));
        },
        (Arr(ar), Value(bv)) => {
            return compare(Arr(ar), Arr(vec![Value(bv)]));
        }
    }
    Ordered
}

pub fn evaluate_sorted(input: &String) -> usize {
    let pairs = read_pairs(input);
    // let hm: Vec<bool> = pairs.iter().map(|(a, b)| compare(a.to_owned(), b.to_owned()) == Compare::Ordered).collect();
    // dbg!(hm);
    pairs.iter().enumerate()
        .filter(|(_, (a, b))| compare(a.to_owned(), b.to_owned()) == Compare::Ordered)
        .fold(0, |acc, (v,(_, _))| (v+1)+acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_tests() {
        dbg!(parse_packet("[[7,[[8,10,1,0,2],0,10]]]"));

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

        // evaluate_sorted(&input);
        assert_eq!(evaluate_sorted(&input), 13);
    }
}