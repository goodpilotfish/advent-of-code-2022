// TODO - REFACTOR
// Replace n*n match wtih Hashset DONE

use std::fs;
use std::error::Error;

mod task {
    use std::collections::HashSet;

    const OFFSET_LOWER: u32 = 96;
    const OFFSET_UPPER: u32 = 38;

    pub fn find_prio(c: char) -> u32 {
        if c.is_lowercase() {
            c as u32 - OFFSET_LOWER
        } else if c.is_uppercase() {
            c as u32 - OFFSET_UPPER
        } else {
            unreachable!()
        }
    }

    pub fn find_match(s: &str) -> char {
        let (first, second) = s.split_at(s.len()/2);
        //println!("{}, {}", first, second);

        let comp_1: HashSet<char> = HashSet::from_iter(first.chars());
        let comp_2: HashSet<char> = HashSet::from_iter(second.chars());

        *comp_1
            .intersection(&comp_2)
            .next()
            .expect("No match: BUG")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;

    let res = file
        .lines()
        .fold(0, |acc, line| {
            let common = task::find_match(line);
            let prio = task::find_prio(common);
            acc + prio
        });
    println!("Total: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_prio() {
        assert_eq!(1, task::find_prio('a'));
        assert_eq!(26, task::find_prio('z'));
        assert_eq!(27, task::find_prio('A'));
        assert_eq!(52, task::find_prio('Z'));
    }

    #[test]
    fn find_match() {
        assert_eq!('p', task::find_match("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }
}