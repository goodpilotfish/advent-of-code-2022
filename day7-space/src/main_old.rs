// Support non unique folder names - TODO

use std::collections::HashMap;
use std::fs;
use std::thread;

#[derive(PartialEq, Debug)]
enum Type {
    FILE,
    DIR
}

#[derive(Debug)]
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
                    //println!("{}", line);
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
fn child_sum(key: &str, hash: &HashMap<String, Vec<Entry>>, sum_hash: &HashMap::<String, u32>) -> u32 {
    println!("Key: {}", key);
    let val = hash.get(key).unwrap();
    if val.iter().all(|x| !x.is_dir()) {
        return val.iter().fold(0, |acc, x| acc + x.size);
    }

    let mut sum = 0;
    for entry in val {
        if entry.is_dir() {
            // optimization - lookup
            if let Some(x) = sum_hash.get(key) {
                sum += x; 
            } else {
                sum += child_sum(&entry.name, hash, sum_hash);
            }
        } else {
            sum += entry.size;
        }
    }
    sum
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

    // run through hashmap
    let mut sum = HashMap::<String, u32>::new();
    loop {
        println!("Size: {}", sum.len());
        // exit condition
        if hashmap.len() == sum.len() {
            break;
        }

        for key in hashmap.keys() {
            let res = child_sum(key, &hashmap, &sum);
            println!("Key: {}, Res: {}", key, res);
            sum.insert(key.to_string(), res);
        }
    };

    sum.values()
        .filter(|x| **x <= 100_000)
        .fold(0, |acc, x| { acc + x})
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let num: u64 = 100_000;
    thread::Builder::new().stack_size(num as usize * 0xFF).spawn(
        move || {
            println!("{}", run(&file));
        }).unwrap().join();
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
