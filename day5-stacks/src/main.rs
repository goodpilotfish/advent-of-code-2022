// TODO - Refactor
// Break out top_three()

mod task {
    pub fn init_stacks(_input: &str, stack: &mut Vec<Vec<char>>) {
        let v1 = vec!['Z', 'N', 'D'];
        let v2 = vec!['M', 'C'];
        let v3 = vec!['P'];

        *stack = vec![v1, v2, v3]
    }

    // move 1 from 2 to 1
    pub fn operate_stacks(input: &str, stack: &Vec<&mut Vec<char>>) -> String {
        input
            .lines()
            .for_each(|line| {
                let tmp: Vec<u32> = line
                    .split(" ")
                    .filter_map(|x| {
                        x.parse::<u32>().ok()
                    }).collect();

                //dbg!(tmp);
                match tmp[..] {
                    [crates, src, target] => {
                        println!("{}, {}, {}", crates, src, target);
                        move_crates((crates, src, target), stack);
                    },
                    _ => unreachable!(),
                };
            }
        );

        String::from("ABC")
    }

    pub fn move_crates(input: (u32, u32, u32), _stack: &Vec<&mut Vec<char>>) {
        input.0;
    }
}

fn main() {
    let mut _stacks: Vec<&mut Vec<char>> = Vec::new();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_stacks() {
        let input = "\
[D]    
[N] [C]    
[Z] [M] [P]
1   2   3 ";
        let mut stacks: Vec<Vec<char>> = Vec::new();

        let v1 = vec!['Z', 'N', 'D'];
        let v2 = vec!['M', 'C'];
        let v3 = vec!['P'];
        let res = vec![v1, v2, v3];

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
        let stacks: Vec<&mut Vec<char>> = Vec::new();

        assert_eq!(String::from("CMZ"), task::operate_stacks(&input, &stacks));
    }

    #[test]
    fn move_crates() {
        let mut v1 = vec!['Z', 'N', 'D'];
        let mut v2 = vec!['M', 'C'];
        let mut v3 = vec!['P'];
        let res = vec![&mut v1, &mut v2, &mut v3];

        task::move_crates((1, 2, 3), &res);

        //assert_eq()
    }
}
