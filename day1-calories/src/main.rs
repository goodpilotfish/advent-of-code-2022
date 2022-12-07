// TODO - REFACTOR
// Break out file load - DONE
// - Opt: max size is calculated during loading for first results DONE
// - Create functional versions DONE
// Find a better data structure
// - Vec can be replaced with BinaryHeap 1to1. If find_max is called often (peek)

use std::fs;
use std::error::Error;

mod calorie_counting {
    pub fn load(input: &str, elfs: &mut Vec::<u32>) -> u32 {
        let mut itr = input.lines();
        let mut sum = 0;
        let mut max_sum = 0;
        while let Some(line) = itr.next() {
            if let true = line.is_empty() {
                elfs.push(sum);
                // task speed up
                if sum > max_sum { max_sum = sum }
                sum = 0;
                continue;
            };

            sum += line.trim().parse::<u32>().unwrap();
        };
        // Hack - push last elf
        if sum > 0 {
            elfs.push(sum);
            if sum > max_sum { max_sum = sum }
        }
        max_sum
    }

    pub fn find_max_innefficent(elfs: &Vec::<u32>) -> u32 {
        elfs.iter().fold(0, |max, x| {
            if x > &max { return *x; } 
            max
        })
    }

    pub fn top_tree_sum(elfs: &mut Vec::<u32>) -> u32 {
        elfs.sort();

        elfs[elfs.len()-1] + elfs[elfs.len()-2] + elfs[elfs.len()-3]
    }
}

mod functional {
    pub fn find_max(input: &str) -> u32 {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    }

    use itertools::Itertools;
    pub fn top_three_sum(input: &str) -> u32 {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut elfs = Vec::<u32>::new();

    let file = fs::read_to_string("input.txt")?;
    println!("Optimization - Max: {}", calorie_counting::load(&file, &mut elfs));

    // Part 1
    println!("Max: {}", elfs.iter().max().unwrap());

    // Part 2
    println!("{}", calorie_counting::top_tree_sum(&mut elfs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_innefficent() {
        let elfs = vec![1, 2, 4, 3];
        assert_eq!(4, calorie_counting::find_max_innefficent(&elfs));
    }

    #[test]
    fn simple_top_tree_sum() {
        let mut elfs = vec![1, 2, 4, 3];
        assert_eq!(9, calorie_counting::top_tree_sum(&mut elfs));
    }

    #[test]
    fn load() {
        let input = "\
2000
3000

10000

2000";
        let mut elfs = Vec::<u32>::new();

        assert_eq!(
            10000,
            calorie_counting::load(input, &mut elfs),
        );
        assert_eq!(
            vec![5000, 10000, 2000],
            elfs
        );
    }
}

#[cfg(test)]
mod tests_functional {
    use super::*;

    #[test]
    fn top_tree_sum() {
        let input = "\
2000
3000

10000

2000";
        assert_eq!(17000, functional::top_three_sum(input));
    }

    #[test]
    fn load() {
        let input = "\
2000
3000

10000

2000";
        assert_eq!(
            10000,
            functional::find_max(input),
        );
    }
}
