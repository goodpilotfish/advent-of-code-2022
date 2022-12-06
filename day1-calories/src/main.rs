// TODO - REFACTOR
// Break out file load - DONE
// Find a better data structure


use std::fs;
use std::error::Error;

mod calorie_counting {
    pub fn load(input: &str, elfs: &mut Vec::<u32>) {
        let mut itr = input.lines();
        let mut sum = 0;
        while let Some(line) = itr.next() {
            if let true = line.is_empty() {
                elfs.push(sum);
                sum = 0;
                continue;
            };

            sum += line.trim().parse::<u32>().unwrap();
        };
        // Hack - push last elf
        if sum > 0 {
            elfs.push(sum);
        }
    }

    pub fn find_max_innefficent(elfs: &Vec::<u32>) -> u32 {
        let mut max = 0;
        let mut elf_index = 0;
        for (i, elf) in elfs.iter().enumerate() {
            if elf > &max {
                max = *elf;
                elf_index = i;
            }
            //println!("Calories: {}, Index: {}", elf, i);
        }
        println!("Most calories: {}, By elf: {}", max, elf_index);
        max 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut elfs = Vec::<u32>::new();

    let file = fs::read_to_string("input.txt")?;
    calorie_counting::load(&file, &mut elfs);
    calorie_counting::find_max_innefficent(&elfs);

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
    fn load() {
        let input = "\
2000
3000

10000";
        let mut elfs = Vec::<u32>::new();

        calorie_counting::load(input, &mut elfs);
        assert_eq!(
            vec![5000, 10000],
            elfs
        );
    }
}
