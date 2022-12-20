use std::str::Lines;
use std::fs;

fn folder_size(reader: &mut Lines, folder_sum: &mut Vec<u32>) -> u32 {
    let mut size = 0;

    while let Some(line) = reader.next() {
        if line == "$ cd .." { break; } // proceed to next folder

        let instr: Vec<&str> = line.split(" ").collect();
        match instr[1] {
            "cd" => size += folder_size(reader, folder_sum),
            _ => size += instr[0].parse::<u32>().unwrap_or(0),
        }
    }
    folder_sum.push(size);

    size
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut sums: Vec<u32> = vec![1, 2];//Vec::new();
    let total = folder_size(&mut file.lines(), &mut sums);
    println!("Total Space Used: {}", total);

    //dbg!(&sums);
    let res = sums
        .iter()
        .filter(|&x| *x < 100_000)
        .fold(0, |acc, x| acc + x);

    println!("Part 1: {}", res);

    // Part 2
    let space = 30_000_000 - (70_000_000 - total);
    sums.sort();
    let f = sums.iter().find(|&&x| x > space);
    println!("Needs to be freed: {}, Folder picked - Part 2: {}", space, f.unwrap());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_size() {
        let task = "\
$ cd /
$ ls
dir a
2 b.txt
2 c.dat
$ cd a
$ ls
1 f";
        let mut sums: Vec<u32> = Vec::new();

        assert_eq!(5, folder_size(&mut task.lines(), &mut sums));
    }
}