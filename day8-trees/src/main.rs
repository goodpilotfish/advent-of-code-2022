use std::fs;

fn is_visible(tree: u8, compare: &[u8]) -> bool {
    !compare
        .iter()
        .any(|&x| x >= tree)
}

fn check_tree((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    let height = grid.len();
    let width = grid[x].len();

    // handle row
    let mut east: Vec<u8> = grid[x].clone();
    let mut west: Vec<u8> = east.split_off(y+1);
    east.pop();

    let flattened: Vec<u8> = grid.into_iter().flatten().cloned().collect();

    // handle column
    let mut column = vec![];
    for (idx, &val) in flattened.iter().enumerate() {
        if idx % width == y { 
            column.push(val);
        };
    }
    let north = &column[0..x];
    let south = &column[x..height-1];

    let res = is_visible(grid[x][y], &east) 
        || is_visible(grid[x][y], &west)
        || is_visible(grid[x][y], &north)
        || is_visible(grid[x][y], &south);
    //println!("Res: {}. ({},{}). Grid: {}", res, x, y, grid[x][y]);
    res
}

fn calculate_border(width: u32, height: u32) -> u32 {
    width*2  + (height - 2) * 2
}

fn run(input: &str) -> u32 {
    // build
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
            //println!("{:?}", row);
            grid.push(row);
        });
  
    // calculate
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for x in 1..=width-2 {
        for y in 1..=height-2 {
            if check_tree((x, y), &grid) {
                sum += 1;
            }
        }
    }
  
    let offset = calculate_border(width as u32, height as u32);
    println!("Offset: {}. W: {}, H: {}", offset, width, height);
    sum+offset
} 

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("Total: {}", run(&file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let tree: u8 = 5;
        let vec_hidden = vec![5, 1, 2];
        let vec1_visible = vec![3, 3, 2];
        
        assert_eq!(false, is_visible(tree, &vec_hidden));
        assert_eq!(true, is_visible(tree, &vec1_visible));
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

        assert_eq!(true, check_tree((1, 1), &grid));
    }

    #[test]
    fn test_run() {
        let input = "\
30373
25512
65332
33549
35390";

        assert_eq!(21, run(&input));
    }
}