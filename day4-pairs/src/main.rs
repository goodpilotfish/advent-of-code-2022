// TODO - reactor
// Can I make the solution be more functional

use std::fs;
use std::error::Error;

mod task {
    use std::collections::HashSet;
    const IS_PART1: bool = true;

    pub fn is_subset(x: &Vec<u32>, y: &Vec<u32>) -> bool {
        let a: HashSet<&u32> = HashSet::from_iter(x.iter());
        let b: HashSet<&u32> = HashSet::from_iter(y.iter());

        if IS_PART1 {
            // Part 1 is looking for subsets
            a.is_subset(&b) || b.is_subset(&a)
        } else {
            // Part 2 us looking for any common sections
            !a.is_disjoint(&b)
        }
    }

    pub fn extract_sections(pair: &str) -> (u32, u32, u32, u32) {
        let (section_a, section_b) = pair.split_once(",").unwrap();
        let (a, b) = section_a.split_once("-").unwrap();
        let (c, d) = section_b.split_once("-").unwrap();
        (
            a.parse::<u32>().unwrap(),
            b.parse::<u32>().unwrap(),
            c.parse::<u32>().unwrap(),
            d.parse::<u32>().unwrap(),
        )
    }

    pub fn run(input: &str) -> u32 {
        input
            .lines()
            .fold(0, |acc, pair| {
                let (aa, bb, cc, dd) = extract_sections(&pair);

                let v1: Vec<u32> = (aa..=bb).collect();
                let v2: Vec<u32> = (cc..=dd).collect();

                acc + if is_subset(&v1, &v2) { 1 } else { 0 }
            })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    println!("Nbr of subsets: {}", task::run(&file));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_subset() {
        let x = vec![2, 3, 4];
        let y = vec![6, 7, 8];
        assert_eq!(false, task::is_subset(&x, &y));
    }

    #[test]
    fn full_subset() {
        let x = vec![6];
        let y = vec![4, 5, 6];
        assert_eq!(true, task::is_subset(&x, &y));
    }

    #[test]
    fn partial_subset() {
        let x = vec![5, 6, 7];
        let y = vec![7, 8, 9];
        assert_eq!(false, task::is_subset(&x, &y));
    }

    #[test]
    fn test_extraction() {
        assert_eq!((1, 2, 3, 4), task::extract_sections("1-2,3-4"));
    }

    #[test]
    fn run() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, task::run(&input));
    }

    #[test]
    fn run_dd() {
        let input = "\
        13-85,12-36";

        assert_eq!(0, task::run(&input));
    }
}
