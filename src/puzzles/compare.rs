
fn read_pairs(input: &String) {
    let vectored: Vec<&str> = input.trim().lines().collect();
    let grouped: Vec<&[&str]> = vectored.split(|s| *s == "").collect();

    grouped.iter().for_each(|group| {
        let first = group[0];
        let second = group[1];
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_tests() {
        dbg!(parse_packet("[[7,[[8,10,1,0,2],0,10]]]"));
    }
}