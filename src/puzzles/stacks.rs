use std::collections::HashMap;

type Stacks = HashMap<u8, Vec<char>>;
type Instruction = (u8, u8, u8);

fn read_arrangement(input: &String) -> (Stacks, Vec<Instruction>) {
    let vectored: Vec<&str> = input.lines().collect();
    let grouped: Vec<&[&str]> = vectored.split(|s| *s == "").collect();

    let boxes: Vec<&str> = grouped.get(0).expect("Must have a box arrangement").iter().map(|l|*l).collect();
    let stack = read_boxes(boxes);

    let is: Vec<&str> = grouped.get(1).unwrap().iter().map(|l|*l).collect();
    let instructions = read_instructions(is);
    (stack, instructions)
}

fn read_boxes(input: Vec<&str>) -> Stacks{
    let indexes = input.iter().rev().next().expect("Must have an element");
    let mut index_map: HashMap<usize, u8> = HashMap::new();
    let index_list = indexes.char_indices().filter(|(_, c)| *c!=' ');

    index_list.clone().for_each(|(i, c)| { index_map.insert(i, c.to_digit(10).expect("Must be a digit") as u8);});

    let char_indexes: Vec<usize> = index_list.map(|(i, _)| i).collect();

    let mut stacks: HashMap<u8, Vec<char>> = HashMap::new();

    input.iter().rev().skip(1).for_each(|line| {
        let cs: Vec<char> = line.chars().collect();
        for index in &char_indexes {
            match cs.get(*index) {
                Some(c) => {
                    if *c==' ' {continue;} 
                    let true_index = index_map.get(index).expect("Index translation must exist");
                    match stacks.get_mut(true_index) {
                        Some(v) => v.push(*c),
                        None => { stacks.insert(*true_index, vec![*c]); }
                    }
                },
                None => ()
            }
        }
    });
    stacks
}

fn read_instructions(input: Vec<&str>) -> Vec<Instruction>{
    input.iter().map(|line| {
        let ls: Vec<&str> = line.split(" ").collect();
        let count: u8 = ls.get(1).unwrap().parse().unwrap();
        let origin: u8 = ls.get(3).unwrap().parse().unwrap();
        let destination: u8 = ls.get(5).unwrap().parse().unwrap();
        (count, origin, destination)
    }).collect()
}

fn process(stacks: &mut Stacks, instruction: Instruction) {
    let (count, origin, destination) = instruction;

    for _ in 0..count {
        let move_me = stacks.get_mut(&origin).expect("origin index must exist").pop().unwrap();
        stacks.get_mut(&destination).expect("Destination index must exist").push(move_me);
    }
}

pub fn full_process(input: &String) -> String {
    let (mut stacks, instructions) = read_arrangement(input);

    for instruction in instructions {
        process(&mut stacks, instruction);
    }

    let mut output: Vec<char> = Vec::new();

    for i in 1..=stacks.len() {
        let c = stacks.get(&(i as u8)).unwrap().last().unwrap();
        output.push(*c);
    }

    output.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hm_behaviour() {
        let mut test: HashMap<u8, Vec<u8>> = HashMap::new();

        match test.get_mut(&1) {
            Some(i) => i.push(2),
            None => { test.insert(1, vec![3]); }
        }

        dbg!(test);
    }

    #[test]
    fn read_test() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ".to_string();

        let out = read_boxes(input.lines().collect());
        dbg!(out);
    }

    #[test]
    fn bigger_read_test() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();

        dbg!(full_process(&input));
    }
}
