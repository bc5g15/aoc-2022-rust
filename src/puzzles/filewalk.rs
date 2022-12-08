use std::collections::HashMap;

type WeightMap = HashMap<String, u32>;

enum Line {
    Move (String),
    MoveUp,
    Directory (String),
    File (String, u32),
    List
}

fn interpret_line(input: &str) -> Line {
    let parts: Vec<&str> = input.split(" ").collect();
    match parts.get(0).expect("Line must be populated") {
        &"$" => {
            match parts.get(1).unwrap() {
                &"cd" => {
                    if parts.get(2) == Some(&"..") {
                        return Line::MoveUp;
                    } else {
                        return Line::Move(parts.get(2).unwrap().to_string());
                    }
                },
                &"ls" => {
                    return Line::List;
                },
                _ => panic!("Unfamiliar command: {input}")
            }
        },
        &"dir" => {
            return Line::Directory(parts.get(1).unwrap().to_string())
        },
        n => {
            if let Ok(size) = n.parse::<u32>() {
                return Line::File(parts.get(1).unwrap().to_string(), size);
            } else {
                panic!("Unfamiliar line: {input}");
            }
        }
    }
}

fn build_path_name(address: &Vec<String>, name: &String) -> String{
    let stack = address.join("/");
    format!("{stack}/{name}")
}


fn traverse_tree(input: &String) -> (WeightMap, WeightMap) {
    let mut weight_stack : Vec<u32> = Vec::new();
    let mut address: Vec<String> = Vec::new();
    let mut weight_sum = 0;

    let mut weights: WeightMap = HashMap::new();
    let mut dir_weights: WeightMap = HashMap::new();

    input.trim().lines().for_each(|line| {
        match interpret_line(line) {
            Line::List => (), // Do nothing
            Line::File(name, size) => {
                let full_addr = build_path_name(&address, &name);
                weights.insert(full_addr, size);
                weight_sum = weight_sum + size;
            },
            Line::Directory(_name) => (), // Do nothing
            Line::Move(directory) => {
                address.push(directory);
                weight_stack.push(weight_sum);
                weight_sum = 0;
            },
            Line::MoveUp => {
                let full_addr = address.join("/");
                dir_weights.insert(full_addr, weight_sum);
                address.pop();
                weight_sum = weight_sum + weight_stack.pop().unwrap();
            }
        }
    });

    // Add the last few weights to the map
    while address.len() > 0 {
        let full_addr = address.join("/");
        dir_weights.insert(full_addr, weight_sum);
        address.pop();
        weight_sum = weight_sum + weight_stack.pop().unwrap();
    }

    (weights, dir_weights)
}

pub fn biggest_small_dirs(input: &String) -> u32 {
    let (_files, dirs) = traverse_tree(&input);

    dirs.values().filter(|v| **v < 100_000).sum()
}

pub fn smallest_big_dir(input: &String) -> u32 {
    let (_files, dirs) = traverse_tree(&input);

    let top_weight = dirs.get("/").unwrap();
    let free_space = 70_000_000 - top_weight;
    let required_space = 30_000_000 - free_space;

    *dirs.values().filter(|v| **v >= required_space).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_weights() {
        let input = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();

        let (_f, d) = traverse_tree(&input);

        dbg!(d.get("/"));

        assert_eq!(biggest_small_dirs(&input), 95437);
    }

    #[test]
    fn part_two() {
        let input = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();

        assert_eq!(smallest_big_dir(&input), 24933642);
    }
}