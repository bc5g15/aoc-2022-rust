use std::collections::{HashMap, VecDeque};

type FileTree = ArenaTree<(String, Option<u32>)>;
type FileElem = Node<(String, Option<u32>)>;

#[derive(Debug)]
struct Node<T> where T: PartialEq {
    idx: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T> where T: PartialEq {
    fn new(idx: usize, value: T) -> Self {
        Self {
            idx,
            value,
            parent: None,
            children: vec![]
        }
    }
}

// impl FileElem {
//     fn get_size(&mut self) -> u32 {

//     }
// }



#[derive(Debug, Default)]
struct ArenaTree<T> where T: PartialEq {
    arena: Vec<Node<T>>
}

impl<T> ArenaTree<T> where T: PartialEq {
    fn node(&mut self, value: T) -> usize {
        // See if it exists
        for node in &self.arena {
            if node.value == value {
                return node.idx;
            }
        }
        // Otherwise add the new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, value));
        idx
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        self.arena[parent].children.push(child);
        self.arena[child].parent = Some(parent);
    }

    fn list_nodes(&self) -> Vec<usize>{
        self.arena.iter().filter(|node| {
            node.children.len() > 0
        }).map(|node| node.idx).collect()
    }
}

impl FileTree {
    // fn get_weight(&mut self, node: usize) -> u32{
    //     let (name, size) = &self.arena[node].value;
    //     match size {
    //         Some(v) => return *v,
    //         None => {
    //             let weight = self.arena[node].children.iter()
    //             .map(|index| {
    //                 self.get_weight(*index)
    //             }).sum();
    //             // self.arena[node].value = (name.to_string(), Some(weight));
    //             return weight;
    //         }
    //     }
    // }

    // fn make_weighted_tree(&self, )
    fn stupid_weight_calculation(&self) -> HashMap<usize, u32>{
        let mut weights: HashMap<usize, u32> = HashMap::new();
        let easy_nodes = self.arena.iter().filter(|n| n.children.len() == 0);

        for node in easy_nodes {
            let (_name, size) = &node.value;
            weights.insert(node.idx, size.unwrap());
        }

        let hard_nodes = self.arena.iter().filter(|n| n.children.len() > 0);
        let mut hard_q = VecDeque::from_iter(hard_nodes);

        while hard_q.len() > 0 {
            let current = hard_q.pop_front().unwrap();
            if current.children.iter().all(|n| weights.contains_key(n)) {
                let my_size = current.children.iter().fold(0, |acc, cur| weights.get(cur).unwrap() + acc);
                weights.insert(current.idx, my_size);
            } else {
                hard_q.push_back(current);
            }
        }
        weights
    }

    fn better_weight_calculation(&self) {
        
    }
}

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

// fn get_weights(files: &FileTree) -> u32{
//     let mut node = files.arena[0];
//     let known_sizes: HashMap<String, u32> = HashSet::new();
//     let (_name, size) = node.value;
//     if let Some(v) = size {
//         return v;
//     } else {
//         node.children.iter().map()
//     }
// }

// fn populate_directory_weights(files: &mut FileTree, node_index: usize) -> u32 {
//     // let node = files.arena[node_index].value;

//     let mut my_size: u32 = 0;

//     for node in &files.arena[node_index].children {
//         let (_name, size) = &files.arena[*node].value;
        
//         if let Some(value) = size {
//             my_size = my_size + value;
//         } else {
//             my_size = my_size + populate_directory_weights(files, *node);
//         }
//     }

//     return my_size;
// }

// fn make_sized_tree(files: &FileTree) {
//     let mut current_node: usize = 0;

//     // Do all the easy calculations
//     for node in &files.arena {

//     }

// }

fn build_tree(input: &String) -> FileTree {
    let mut files: ArenaTree<(String, Option<u32>)> = ArenaTree::default();
    let mut current_directory: Option<usize> = None;
    // let mut sizes: HashMap<String, u32> = HashMap::new();
    // let mut file_parents: HashMap<String, String> = HashMap::new();
    // let mut dir_children: HashMap<String, Vec<String>> = HashMap::new();
    // let mut current_directory: String = "/".to_string();

    // let add_child = |name: String| {
    //     file_parents.insert(name, &current_directory);
    //     match dir_children.get_mut(&current_directory) {
    //         Some(v) => v.push(name),
    //         None => { dir_children.insert(current_directory, vec![name]);}
    //     }
    // };

    // let mut add_child = |name: String, size: Option<u32>| {
    //     let node_id = files.node((name, size));
    //     files.arena[node_id].parent = current_directory;
    //     if let Some(dir) = current_directory {
    //         files.arena[dir].children.push(node_id);
    //     }
    // };

    input.trim().lines().for_each(|line| {
        match interpret_line(line) {
            Line::List => (), // Do nothing
            Line::File(name, size) => {
                // files.arena[current_directory].children.push()
                // sizes.insert(name, size);
                let new_node = files.node((name, Some(size)));
                if let Some(dir) = current_directory {
                    files.add_child(dir, new_node);
                }
                // add_child(name, Some(size));

            },
            Line::Directory(name) => {
                let new_node = files.node((name, None));
                if let Some(dir) = current_directory {
                    files.add_child(dir, new_node);
                }
            },
            Line::Move(directory) => {
                current_directory = Some(files.node((directory, None)));
            },
            Line::MoveUp => {
                if let Some(v) = files.arena[current_directory.unwrap()].parent {
                    current_directory = Some(v);
                }
            }
        }
    });
    files
}

pub fn biggest_small_weights(input: &String) -> u32 {
    let tree = build_tree(input);
    dbg!(tree);
    // let weights = tree.stupid_weight_calculation();
    // let dirs = tree.list_nodes();
    // dirs.iter().map(|id| *weights.get(id).unwrap()).filter(|i| *i < 100_000).sum()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let mut test: ArenaTree<(String, u8)> = ArenaTree::default();
        test.node(("a".into(), 1));
        test.node(("b".into(), 2));

        dbg!(test.node(("a".into(), 1)));
        dbg!(test.node(("c".into(), 3)));

    }

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

        assert_eq!(biggest_small_weights(&input), 95437);
    }
}
