use std::collections::{HashMap, VecDeque};

enum Line {
    Move (String),
    MoveUp,
    Directory (String),
    File (String, u32),
    List
}

type SizeMap = HashMap<String, u32>;
type ParentMap = HashMap<String, String>;
type ChildMap = HashMap<String, Vec<String>>;

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

fn build_name(address: &Vec<String>, node: &String) -> String {
    let root = address.join("/");
    format!("{root}/{node}")
}

fn build_tree(input: &String) -> (SizeMap, ParentMap, ChildMap) {
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut file_parents: HashMap<String, String> = HashMap::new();
    let mut dir_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_node:  String = "/".to_string();
    let mut address: Vec<String> = vec!["/".to_string()];

    input.trim().lines().for_each(|line| {
        match interpret_line(line) {
            Line::List => (), // Do nothing
            Line::File(name, size) => {
                sizes.insert(build_name(&address, &name), size);
                file_parents.insert(name.to_string(), current_node.to_string());
                match dir_children.get_mut(&current_node) {
                    Some(v) => v.push(name.to_string()),
                    None => { dir_children.insert(current_node.to_string(), vec![name.to_string()]); }
                }
            },
            Line::Directory(name) => {
                file_parents.insert(name.to_string(), current_node.to_string());
                match dir_children.get_mut(&current_node) {
                    Some(v) => v.push(name.to_string()),
                    None => { dir_children.insert(current_node.to_string(), vec![name.to_string()]); }
                }
            },
            Line::Move(directory) => {
                address.push(current_node.to_string());
                current_node = directory;
            },
            Line::MoveUp => {
                address.pop();
                let upper_name = file_parents.get(&current_node).unwrap();
                current_node = upper_name.to_string();
                // current_node = *file_parents.get(&current_node.to_string()).unwrap().to_string();
            }
        }
    });
    (sizes, file_parents, dir_children)
}

fn find_directory_sizes(sizes: &SizeMap, parents: &ParentMap, children: &ChildMap) -> SizeMap {
    let root_node = children.get(&"/".to_string()).unwrap();
    
    let mut dir_sizes: SizeMap = HashMap::new();

    let mut op_stack: Vec<(String, u32, Vec<String>)> = Vec::new();

    let mut my_children: Vec<String> = root_node.clone();

    let mut weight = 0;
    let mut my_dir: String = "/".to_string();

    loop {
        match my_children.pop() {
            Some(child) => {
                dbg!(&my_dir);
                dbg!(&child);
                match sizes.get(&child) {
                    Some(w) => {
                        dbg!("Got size");
                        weight = weight + w;
                    },
                    None => {
                        if let Some(v) = dir_sizes.get(&child) {
                            dbg!("Remembered size");
                            weight = weight + v;
                        } else {
                            my_children.push(child.to_string());
                            op_stack.push((my_dir, weight, my_children));
                            my_dir = child.to_string();
                            my_children = children.get(&child.to_string()).unwrap().clone();
                            weight = 0;
                        }
                    }
                }
            },
            None => {
                // No more children, assign weight and move up the stack
                dir_sizes.insert(my_dir.to_string(), weight);

                match op_stack.pop() {
                    Some ((old_dir, old_weight, old_child)) => {
                        my_dir = old_dir;
                        weight = old_weight;
                        my_children = old_child;
                    },
                    None => {
                        // Nothing left on the stack. We are done
                        return dir_sizes;
                    }
                }
            }
        }
    }
}

pub fn biggest_small_dirs(input: &String) -> u32 {
    let (a, b, c) = build_tree(input);
    dbg!(&c);
    dbg!(&a);
    let dir_sizes = find_directory_sizes(&a, &b, &c);
    dbg!(&dir_sizes);
    dir_sizes.values().filter(|v| **v < 100_000).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
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

        assert_eq!(biggest_small_dirs(&input), 95437);
    }
}
