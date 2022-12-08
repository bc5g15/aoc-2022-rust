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
    // let mut sizes: HashMap<String, u32> = HashMap::new();
    // let mut file_parents: HashMap<String, String> = HashMap::new();
    // let mut dir_children: HashMap<String, Vec<String>> = HashMap::new();
    // let mut current_node:  String = "/".to_string();
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
                // sizes.insert(name.to_string(), size);
                // file_parents.insert(name.to_string(), current_node.to_string());
                // match dir_children.get_mut(&current_node) {
                //     Some(v) => v.push(name.to_string()),
                //     None => { dir_children.insert(current_node.to_string(), vec![name.to_string()]); }
                // }
            },
            Line::Directory(_name) => {
                // Nothing yet
            },
            Line::Move(directory) => {
                // current_node = directory;
                address.push(directory);
                weight_stack.push(weight_sum);
                weight_sum = 0;
            },
            Line::MoveUp => {
                let full_addr = address.join("/");
                dir_weights.insert(full_addr, weight_sum);
                address.pop();
                weight_sum = weight_sum + weight_stack.pop().unwrap();
                // let upper_name = file_parents.get(&current_node).unwrap();
                // current_node = upper_name.to_string();
                // current_node = *file_parents.get(&current_node.to_string()).unwrap().to_string();
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
    // (sizes, file_parents, dir_children)
}

pub fn biggest_small_dirs(input: &String) -> u32 {
    let (_files, dirs) = traverse_tree(&input);

    dirs.values().filter(|v| **v < 100_000).sum()
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

        dbg!(traverse_tree(&input));

        assert_eq!(biggest_small_dirs(&input), 95437);
    }
}