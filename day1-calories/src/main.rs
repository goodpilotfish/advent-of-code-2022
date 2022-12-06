// TODO - REFACTOR
// Break out file load
// Find a better data structure


use std::fs;
use std::error::Error;

// N*N
pub fn max_calories_ineffective(input: &str) -> u32 {
    let mut elfs = Vec::<u32>::new();

    // STORE
    let mut itr = input.lines();
    let mut sum = 0;
    while let Some(line) = itr.next() {
        if let true = line.is_empty() {
            elfs.push(sum);
            sum = 0;
            continue;
        };

        sum += line.trim().parse::<u32>().unwrap();
        //println!("{}", line);
    };
    // Hack - push last elf
    if sum > 0 {
        elfs.push(sum);
    }

    // FIND
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

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;

    max_calories_ineffective(&file);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(24000, max_calories_ineffective(input));
    }
}
