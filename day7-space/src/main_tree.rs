// Parse instrutions
// Store in tree
// Calculate tree recursivly
// Calculate top

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
enum Type {
    FILE,
    DIR
}

#[derive(PartialEq, Debug)]
struct Entry {
    size: u32,
    name: String,
    file_type: Type,
    parent: Option<Rc<RefCell<Entry>>>,
    children: Vec<Rc<RefCell<Entry>>>,
}

impl Entry {
    fn is_dir(&self) -> bool {
        self.file_type == Type::DIR
    }
}

fn parse_instruction<'a>(instruction: &'a str, root: Rc<RefCell<Entry>>, current: Rc<RefCell<Entry>>) {
    let mut itr = instruction.lines();
    let mut ops = itr.next().unwrap().split_whitespace().filter(|&x| !x.contains('$') );

    match ops.next() {
        Some("cd") => {
            let target = ops.next().unwrap().clone();
            if target == ".." { // go up
                return parent.parent.unwrap()
            } if target == "/" { // stay
                return parent
            } else { // go down

                return parent
            }
        },
        Some("ls") => {
            itr
                .for_each(|line| {
                    //println!("{}", line);
                    let v: Vec<&str> = line.split_whitespace().collect();
                    if v[0] == "dir" {
                    } else {
                    };
                });
        },
        _ => unreachable!(),
    };
    parent
}

fn run(instructions: &str) -> u32 {
    42
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_instruction() {
        let mut tree = Tree { root: Node {
            parent: &None,
            children: Vec::new(),
            value: Some( Entry {
                size: 0,
                name: "/".to_string(),
                file_type: Type::DIR,
            }),
        }};
        let empty = "";
    }

    #[test]
    fn test_ls_instruction() {
        let empty = "$ ls";
        let not_empty = "\
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d";
        let mut tree = Tree { root: None };

    }

    #[test]
    fn run_test() {
        let task = "\
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
7214296 k";

        assert_eq!(95437, run(task));
    }
}