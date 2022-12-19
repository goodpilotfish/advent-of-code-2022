// TODO

use std::collections::HashMap;

#[derive(PartialEq)]
enum Type {
    FILE,
    DIR
}

struct Entry {
    size: u32,
    name: String,
    file_type: Type,
}

impl Entry {
    fn is_dir(&self) -> bool {
        self.file_type == Type::DIR
    }
}

fn parse_instruction<'a>(instruction: &'a str, last_folder: &'a str, hash: &mut HashMap<String, Vec<Entry>>) -> &'a str {
    let mut itr = instruction.lines();
    let mut ops = itr.next().unwrap().split_whitespace().filter(|&x| !x.contains('$') );

    match ops.next() {
        Some("cd") => {
            let target = ops.next().unwrap().clone();
            if target == ".." {
                return last_folder
            } else {
                return target
            }
        },
        Some("ls") => {
            itr
                .for_each(|line| {
                    println!("{}", line);
                    let v: Vec<&str> = line.split_whitespace().collect();
                    if v[0] == "dir" {
                        hash.entry(last_folder.to_string())
                            .or_default()
                            .push(Entry {
                                size: 0,
                                name: v[1].to_string(),
                                file_type: Type::DIR,
                            });
                    } else {
                        hash.entry(last_folder.to_string())
                            .or_default()
                            .push(Entry {
                                size: v[0].parse::<u32>().unwrap(),
                                name: v[1].to_string(),
                                file_type: Type::FILE,
                            });
                    };
                });
        },
        _ => unreachable!(),
    };
    last_folder
}

// recursivly find file sums
fn child_sum(key: &str, hash: &HashMap<String, Vec<Entry>>) -> u32 {
    let val = hash.get(key).unwrap();
    if val.iter().all(|x| !x.is_dir()) {
        return val.iter().fold(0, |acc, x| acc + x.size);
    }

    let folder_entry = val.iter().find(|x| x.is_dir()).unwrap();
    let sum = val.iter()
        .filter(|x| !x.is_dir())
        .fold(0, |acc, x| acc + x.size);
    child_sum(&folder_entry.name, hash) + sum
}

fn run(instructions: &str) -> u32 {
    let mut hashmap = HashMap::<String, Vec<Entry>>::new();

    // build data structure
    let mut last_folder = "";
    instructions
        .split("$")
        .filter(|&x| !x.is_empty())
        .for_each(|instr| {
            last_folder = parse_instruction(instr, last_folder, &mut hashmap);
        });

    // run through hashmap - TODO
    let mut sum = HashMap::<String, u32>::new();
    loop {
        println!("Size: {}", hashmap.len());
        // exit condition
        if hashmap.len() == sum.len() {
            break;
        }

        for key in hashmap.keys() {
            let res = child_sum(key, &hashmap);
            sum.insert(key.to_string(), res);
        }
    };
    dbg!(&sum);

    // TODO write functionall
    let mut res = 0;
    for (_key, val) in &sum {
        if *val <= 100000 {
            res += val
        }
    }
    res
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_instruction() {
        let mut hashmap = HashMap::<String, Vec<Entry>>::new();
        let empty = "";
        assert_eq!("/", parse_instruction("$ cd /", empty, &mut hashmap));
        assert_eq!(0, hashmap.len());

        assert_eq!("a", parse_instruction("$ cd a", "/", &mut hashmap));
        assert_eq!(0, hashmap.len());

        assert_eq!("a", parse_instruction("$ cd ..", "a", &mut hashmap));
        assert_eq!(0, hashmap.len());
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
        let mut hashmap = HashMap::<String, Vec<Entry>>::new();

        assert_eq!("a", parse_instruction(empty, "a", &mut hashmap));
        assert_eq!(0, hashmap.len());

        assert_eq!("a", parse_instruction(not_empty, "a", &mut hashmap));
        assert_eq!(1, hashmap.len());
        assert_eq!(4, hashmap.get("a").unwrap().len());
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
