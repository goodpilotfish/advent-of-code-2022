use std::fs;

fn nbr_visible_trees(tree: u8, compare: &[u8]) -> usize {
    let res = compare
        .iter()
        .take_while(|x| **x < tree)
        .collect::<Vec<_>>()
        .len();

    // edge case: if res is smaller then the length of compare
    // it means we have found a bigger or equal tree. This tree
    // needs to be added, therefore adding 1.
    if res == compare.len() {
        res
    } else {
        res + 1
    }
}

// note: west and north directions needs to be reversed before processing
fn nbr_visible_trees_rev(tree: u8, compare: &[u8]) -> usize {
    let res = compare
        .iter()
        .rev()
        .take_while(|x| **x < tree)
        .collect::<Vec<_>>()
        .len();
    println!("{:?}, len: {}", compare, res);
    if res == compare.len() {
        println!("Res: {}", res);
        res
    } else {
        println!("Res+1: {}", res + 1);
        res + 1
    }
}

fn part2(tree: u8, east: &[u8], west: &[u8], north: &[u8], south: &[u8]) -> u32 {
    let res = nbr_visible_trees_rev(tree, &east) as u32
        * nbr_visible_trees(tree, &west) as u32
        * nbr_visible_trees_rev(tree, &north) as u32
        * nbr_visible_trees(tree, &south) as u32;
    println!("Res: {}. Tree: {}", res, tree);
    res
}

fn check_tree((x, y, width, height): (usize, usize, usize, usize), grid: &Vec<Vec<u8>>) -> u32 {
    // handle row
    let mut east: Vec<u8> = grid[y].clone();
    let west: Vec<u8> = east.split_off(x+1);
    east.pop();

    // handle column
    let flattened: Vec<u8> = grid.into_iter().flatten().cloned().collect();
    let mut column = vec![];
    for (idx, &val) in flattened.iter().enumerate() {
        if idx % width == x { 
            column.push(val);
        };
    }
    let north = &column[0..y];
    let south = &column[y+1..height];

    part2(grid[y][x], &east, &west, &north, &south)
}

fn calculate_border(width: u32, height: u32) -> u32 {
    width*2  + (height - 2) * 2
}

fn run(input: &str) -> u32 {
    // build grid
    let mut grid = Vec::new();
    input
        .lines()
        .for_each(|line| {
            let row = line
                .chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as u8
                })
                .collect::<Vec<_>>();
            grid.push(row);
        });
  
    // calculate
    let width = grid[0].len();
    let height = grid.len();
    //dbg!(&grid);
    let mut max_scenic_score = 0;
    for x in 1..=width-2 {
        for y in 1..=height-2 {
            let res = check_tree((y, x, width, height), &grid); 
            if res > max_scenic_score {
                max_scenic_score = res;
            }
        }
    }
    println!("Max: {}", max_scenic_score);
  
    max_scenic_score
} 

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("Max: {}", run(&file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visble_trees() {
        let tree: u8 = 5;
        let vec_one = vec![5, 1, 2];
        let vec1_three = vec![3, 3, 2];
        
        assert_eq!(1, nbr_visible_trees(tree, &vec_one));
        assert_eq!(3, nbr_visible_trees(tree, &vec1_three));
    }

    #[test]
    fn test_calculate_border() {
        assert_eq!(16, calculate_border(5, 5));
        assert_eq!(10, calculate_border(5, 2));
        assert_eq!(10, calculate_border(2, 5));
        assert_eq!(12, calculate_border(5, 3));
        assert_eq!(12, calculate_border(3, 5));
    }

    #[test]
    fn test_check_tree() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2], 
            vec![2, 5, 5, 1, 2], 
        ];

        assert_eq!(1, check_tree((1, 1, 5, 3), &grid));
    }

    #[test]
    fn test_run() {
        let input = "\
30373
25512
65332
33549
35390";

        assert_eq!(8, run(&input));
    }
}