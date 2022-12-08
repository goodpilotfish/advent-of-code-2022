// TODO - REFACTOR
// Break out file load - DONE
// - Opt: max size is calculated during loading for first results DONE
// - Create functional versions DONE

use std::fs;
use std::error::Error;

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
    let file = fs::read_to_string("input.txt")?;
    // Part 1
    println!("Max: {}", functional::find_max(&file));

    // Part 2
    println!("Top 3 sum: {}", functional::top_three_sum(&file));

    Ok(())
}

#[cfg(test)]
mod tests_functional {
    use super::*;

    #[test]
    fn top_three_sum() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_top_tree_sum() {
        let mut elfs = vec![1, 2, 4, 3];
        assert_eq!(9, calorie_counting::top_three_sum(&mut elfs));
    }

    #[test]
    fn find_max() {
        let input = "\
2000
3000

10000

2000";
        let mut elfs = Vec::<u32>::new();

        assert_eq!(
            10000,
            calorie_counting::find_max(input, &mut elfs),
        );
        assert_eq!(
            vec![5000, 10000, 2000],
            elfs
        );
    }
}
}

// Early version - Dead code
mod calorie_counting {
    #[allow(dead_code)]
    pub fn find_max(input: &str, elfs: &mut Vec::<u32>) -> u32 {
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
        // Remember - push last elf
        if sum > 0 {
            elfs.push(sum);
            if sum > max_sum { max_sum = sum }
        }
        max_sum
    }

    #[allow(dead_code)]
    pub fn top_three_sum(elfs: &mut Vec::<u32>) -> u32 {
        elfs.sort();

        elfs[elfs.len()-1] + elfs[elfs.len()-2] + elfs[elfs.len()-3]
    }
}
