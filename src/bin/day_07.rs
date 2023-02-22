use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File as IOFile;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

const TOTAL_DISK_SIZE: u64 = 70_000_000;
const UPDATE_SIZE: u64 = 30_000_000;

enum Command {
    ChangeDirectory(String),
    List,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.split_whitespace().collect::<Vec<&str>>();
        if s.len() == 0 {
            return Err(());
        }
        match components[0] {
            "cd" => Ok(Command::ChangeDirectory(components[1].to_owned())),
            "ls" => Ok(Command::List),
            _ => Err(()),
        }
    }
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    children: HashMap<String, Rc<RefCell<Node>>>,
    size: usize,
}

impl Node {
    fn new(parent: Option<Rc<RefCell<Node>>>, size: usize) -> Self {
        return Node {
            parent,
            children: HashMap::new(),
            size,
        };
    }

    fn is_dir(&self) -> bool {
        return self.size == 0 && !self.children.is_empty();
    }

    fn total_size(&self) -> u64 {
        return self
            .children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64;
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f
            .debug_struct("Node")
            .field("children", &self.children)
            .field("size", &self.size)
            .finish();
    }
}

fn all_dirs(n: Rc<RefCell<Node>>) -> Box<dyn Iterator<Item = Rc<RefCell<Node>>>> {
    // clippy is wrong and should feel bad
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

fn main() {
    let root = Rc::new(RefCell::new(Node::new(None, 0)));
    let mut cwd = root.clone();
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./input/test_07.txt") {
    if let Ok(lines) = read_lines("./input/input_07.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("$") {
                    println!("This is a command {}", ip);
                    // Dealing with commands
                    if let Some(command) = ip.strip_prefix("$ ") {
                        match Command::from_str(command).unwrap() {
                            Command::ChangeDirectory(dir) => match dir.as_str() {
                                "/" => {}
                                ".." => {
                                    let parent = cwd.borrow().parent.clone().unwrap();
                                    cwd = parent;
                                }
                                _ => {
                                    let child = cwd
                                        .borrow_mut()
                                        .children
                                        .entry(dir)
                                        .or_insert(Rc::new(RefCell::new(Node::new(
                                            Some(Rc::clone(&cwd)),
                                            0,
                                        ))))
                                        .clone();
                                    cwd = child;
                                }
                            },
                            Command::List => println!("List dir"),
                        }
                    }
                } else {
                    if ip.starts_with("dir ") {
                        let dir_name = ip.strip_prefix("dir ").unwrap();
                        println!("This is a directory: {}", dir_name);
                        cwd.borrow_mut()
                            .children
                            .entry(dir_name.to_owned())
                            .or_insert(Rc::new(RefCell::new(Node::new(Some(Rc::clone(&cwd)), 0))));
                    } else {
                        let file_info: Vec<_> = ip.split(" ").collect();
                        println!("This is a file: {}", ip);
                        cwd.borrow_mut()
                            .children
                            .entry(file_info[1].to_owned())
                            .or_insert(Rc::new(RefCell::new(Node::new(
                                Some(Rc::clone(&cwd)),
                                file_info[0].to_owned().parse::<usize>().unwrap(),
                            ))));
                    }
                }
            }
        }
    }
    // println!("{root:#?}");
    let mut sizes = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .collect::<Vec<u64>>();
    sizes.sort();
    sizes.reverse();
    if let Some((root_size, rest)) = sizes.split_first() {
        let mut smallest_deletable_dir_size = &std::u64::MAX;
        for size in rest {
            if TOTAL_DISK_SIZE - root_size + size > UPDATE_SIZE
                && size < smallest_deletable_dir_size
            {
                smallest_deletable_dir_size = size;
            }
        }
        println!("Smallest deletable directory size: {smallest_deletable_dir_size:#?}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<IOFile>>>
where
    P: AsRef<Path>,
{
    let file = IOFile::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
