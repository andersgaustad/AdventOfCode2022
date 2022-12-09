use std::{collections::{HashMap}, fs::File, io::{BufReader, BufRead}};

// Shoutout to https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust

struct DirectoryNode {
    pub name : String,
    pub files : HashMap<String, u32>,
    pub sub_dirs : Vec<DirectoryNode>,
}

impl DirectoryNode {
    fn new(name: String) -> Self {
        Self { name, files: HashMap::new(), sub_dirs: vec![]}
    }

    fn add_child(&mut self, child: DirectoryNode) {
        self.sub_dirs.push(child);
    }
    
    
    fn get_total_size(&self) -> u32 {
        let file_sum = self.files.values().fold(0, |acc, x| acc + x);
        let recursive_sum = self.sub_dirs.iter().fold(file_sum, |acc, dir| {
            acc + dir.get_total_size()
        });

        return recursive_sum;
    }

    fn get_dir_index(&self, name: &str) -> Option<usize> {
        self.sub_dirs.iter().position(|x| x.name == name)
    }

    fn get_sub_dirs_tree(&self) -> Vec<&DirectoryNode> {
        let mut all_dirs = vec![self];

        for sub_dir in self.sub_dirs.iter() {
            let sub_sub_dirs = sub_dir.get_sub_dirs_tree();
            for sub_sub_dir in sub_sub_dirs {
                all_dirs.push(sub_sub_dir);
            }
        } 

        return all_dirs;
    }

    fn to_zipper(self) -> NodeZipper {
        NodeZipper { node: self, parent: None, index_in_parent: 0 }
    }
}

struct NodeZipper {
    node: DirectoryNode,
    parent: Option<Box<NodeZipper>>,
    index_in_parent: usize
}

impl NodeZipper {
    fn child(mut self, index: usize) -> NodeZipper {
        let child = self.node.sub_dirs.swap_remove(index);
        let focused_zipper = NodeZipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index
        };

        // Focus on child
        return focused_zipper;
    }

    fn parent(self) -> NodeZipper {
        // Destructure self
        let NodeZipper { node, parent, index_in_parent } = self;

        // Destructure parent
        let NodeZipper {
            node: mut parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        // Inverse the swap-remove
        parent_node.sub_dirs.push(node);
        let last_index = parent_node.sub_dirs.len() - 1;
        parent_node.sub_dirs.swap(index_in_parent, last_index);

        // Focus on parent
        let focused_zipper = NodeZipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        };

        return focused_zipper;
    }


}

pub fn main() {
    let mut super_root = DirectoryNode::new("".to_string());
    let root = DirectoryNode::new("/".to_string());
    super_root.add_child(root);

    let mut zipper = super_root.to_zipper();

    let mut read_mode = false;
    let file = File::open("res/dec07_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        // Uncomment me for debugging
        // println!("-> {}", &line);

        const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let first = tokens.get(0).expect(WRONG_FORMAT_MESSSAGE);
        match first {
            &"$" => {
                read_mode = false;
                let command = tokens.get(1).expect(WRONG_FORMAT_MESSSAGE);
                match command {
                    &"cd" => {
                        let arg = tokens.get(2).expect(WRONG_FORMAT_MESSSAGE);

                        match arg {
                            &".." => {
                                zipper = zipper.parent();
                            },

                            _ => {
                                let sub_dir_index = zipper.node.get_dir_index(arg).expect("Error finding dir name");
                                zipper = zipper.child(sub_dir_index);
                            } 
                        }  
                    },
    
                    &"ls" => {
                        read_mode = true;
                    },
    
                    _ => panic!("Unrecognized command!")
                }
            },

            &"dir" => {
                assert!(read_mode);
                let name = tokens.get(1).expect(WRONG_FORMAT_MESSSAGE);
                let new_dir = DirectoryNode::new(name.to_string());
                zipper.node.add_child(new_dir);
            },

            _ => {
                assert!(read_mode);
                let size = first.parse::<u32>().expect(WRONG_FORMAT_MESSSAGE);
                let filename = tokens.get(1).expect(WRONG_FORMAT_MESSSAGE).to_string();

                zipper.node.files.entry(filename.to_string()).or_insert(size);
            }
        }
    }

    // Reset
    while zipper.node.name != "/" {
        zipper = zipper.parent();
    }

    let root = zipper.node;

    const BIG_DIR_SIZE_LIMIT : u32 = 100 * 1000;
    let mut sum = 0;
    let all_dirs = root.get_sub_dirs_tree();
    for dir in all_dirs.iter() {
        let dir_size = dir.get_total_size();
        
        if dir_size <= BIG_DIR_SIZE_LIMIT {
            sum += dir_size;
        }
    }

    const TOTAL_DISK_SPACE : u32 = 70 * 1000 * 1000;
    const REQUIRED_SPACE : u32 = 30 * 1000 * 1000;
    let used_space = all_dirs.iter().find(|x| x.name == "/").unwrap().get_total_size();
    let unused_space = TOTAL_DISK_SPACE - used_space;
    let min_delete_size = REQUIRED_SPACE - unused_space;
    let min = all_dirs.iter().map(|dir| dir.get_total_size())
        .filter(|&size| size >= min_delete_size)
        .min().unwrap();

    println!("Sum of dirs with max size {}: {}", &BIG_DIR_SIZE_LIMIT, &sum);
    println!("Min dir size that can be deleted: {}", &min);
}
