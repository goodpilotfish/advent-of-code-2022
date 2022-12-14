// TODO - Refactor
// Break out top_three() DONE
// Read input stacks dynamically DONE
// more functional. Use collect_tuple from Itertools

use std::fs;
use std::error::Error;

mod task {
    pub fn init_stacks(input: &str, stack: &mut Vec<Vec<char>>) {
        let crates = input
            .lines()
            .take_while(|line| !line.is_empty());

        let nbr_stacks = crates
            .clone()
            .last()
            .unwrap()
            .split_whitespace()
            .count();

        let mut stacks = vec![vec![]; nbr_stacks];
        crates.for_each(|line| {
            line.char_indices()
                .filter(|(_, crate_symbol)| crate_symbol.is_alphabetic())
                .for_each(|(i, crate_symbol)| stacks[i / 4].push(crate_symbol))
        });
        stacks.iter_mut().for_each(|x| x.reverse());

        *stack = stacks;
    }

    pub fn operate_stacks(input: &str, stack: &mut Vec<Vec<char>>) -> String {
        input
            .lines()
            .skip_while(|line| !line.contains("move"))
            .for_each(|line| {
                let tmp: Vec<u32> = line
                    .split(" ")
                    .filter_map(|x| {
                        x.parse::<u32>().ok()
                    }).collect();

                // could use collect_tuple() from Itertools
                match tmp[..] {
                    [crates, src, target] => {
                        *stack = move_crates((crates, src-1, target-1), stack);
                    },
                    _ => unreachable!(),
                };
            }
        );

        top_three(&stack)
    }

    pub fn top_three(stack: &Vec<Vec<char>>) -> String {
        stack
            .iter()
            .map(|x| {
                x.last().unwrap().clone()
            }).collect()
    }

    pub fn move_crates(input: (u32, u32, u32), stack: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (nbr_crates, src, target) = input;
        for _ in 0..nbr_crates {
            if let Some(top) = stack[src as usize].pop() {
                stack[target as usize].push(top);
            }
        }

        stack.to_vec()
    }

    #[allow(dead_code)]
    pub fn move_crates_part2(input: (u32, u32, u32), stack: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (nbr_crates, src, target) = input;
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..nbr_crates {
            if let Some(top) = stack[src as usize].pop() {
                tmp.push(top);
            }
        }

        for _ in 0..nbr_crates { 
            if let Some(top) = tmp.pop() {
                stack[target as usize].push(top);
            }
        }

        stack.to_vec()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    task::init_stacks(&file, &mut stacks);
   
    let s = task::operate_stacks(&file, &mut stacks);
    println!("{:?}", s);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_stacks() {
        let input = "\
[N]             [R]             [C]
[T] [J]         [S] [J]         [N]
[B] [Z]     [H] [M] [Z]         [D]
[S] [P]     [G] [L] [H] [Z]     [T]
[Q] [D]     [F] [D] [V] [L] [S] [M]
[H] [F] [V] [J] [C] [W] [P] [W] [L]
[G] [S] [H] [Z] [Z] [T] [F] [V] [H]
[R] [H] [Z] [M] [T] [M] [T] [Q] [W]
1   2   3   4   5   6   7   8   9 ";
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let v1 = vec!['R', 'G', 'H', 'Q', 'S', 'B', 'T', 'N'];
        let v2 = vec!['H', 'S', 'F', 'D', 'P', 'Z', 'J'];
        let v3 = vec!['Z', 'H', 'V'];
        let v4 = vec!['M', 'Z', 'J', 'F', 'G', 'H'];
        let v5 = vec!['T', 'Z', 'C', 'D', 'L', 'M', 'S', 'R'];
        let v6 = vec!['M', 'T', 'W', 'V', 'H', 'Z', 'J'];
        let v7 = vec!['T', 'F', 'P', 'L', 'Z'];
        let v8 = vec!['Q', 'V', 'W', 'S'];
        let v9 = vec!['W', 'H', 'L', 'M', 'T', 'D', 'N', 'C'];

        let res = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];

        task::init_stacks(&input, &mut stacks);

        assert_eq!(res, stacks);
    }

    #[test]
    fn operate_stacks() {
        let input = "\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"; 
        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];
        let mut stacks = vec![v1, v2, v3];

        assert_eq!(String::from("CMZ"), task::operate_stacks(&input, &mut stacks));
    }

    #[test]
    fn move_crates() {
        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];
        let mut input = vec![v1, v2, v3];

        let res: Vec<Vec<char>> = task::move_crates((1, 1, 0), &mut input);

        let v1 = vec!['Z', 'N', 'D'];
        let v2 = vec!['M', 'C'];
        let v3 = vec!['P'];
        let output = vec![v1, v2, v3];
        assert_eq!(output, res);
    }

    #[test]
    fn top_three() {
        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];
        let input = vec![v1, v2, v3];

        assert_eq!("NDP", task::top_three(&input));
    }
}
