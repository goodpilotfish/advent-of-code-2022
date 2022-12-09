// TODO - Refactor
// Break out top_three()

mod task {
    pub fn init_stacks(_input: &str, stack: &mut Vec<Vec<char>>) {
        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];

        *stack = vec![v1, v2, v3]
    }

    // move 1 from 2 to 1
    pub fn operate_stacks(input: &str, stack: &mut Vec<Vec<char>>) -> String {
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
                        *stack = move_crates((crates, src-1, target-1), stack);
                    },
                    _ => unreachable!(),
                };
            }
        );

        dbg!(stack);

        String::from("ABC")
    }

    // move 1 from 2 to 1
    pub fn move_crates(input: (u32, u32, u32), stack: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (nbr_crates, src, target) = input;
        let mut tmp = Vec::<char>::new();
        for _ in 0..nbr_crates { // POP
            if let Some(top) = stack[src as usize].pop() {
                println!("Pop: {top}");
                tmp.push(top);
            }
        }

        for _ in 0..nbr_crates { // PUSH
            if let Some(top) = tmp.pop() {
                println!("Push: {top}");
                stack[target as usize].push(top);
            }
        }

        stack.to_vec()
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

        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];
        let res = vec![v1, v2, v3];

        task::init_stacks(&input, &mut stacks);

        assert_eq!(res, stacks);
    }

    #[test]
    #[ignore]
    fn operate_stacks() {
        let input = "\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"; 
        let mut stacks: Vec<Vec<char>> = Vec::new();

        assert_eq!(String::from("CMZ"), task::operate_stacks(&input, &mut stacks));
    }

    #[test]
    fn move_crates() {
        let v1 = vec!['Z', 'N'];
        let v2 = vec!['M', 'C', 'D'];
        let v3 = vec!['P'];
        let mut input = vec![v1, v2, v3];

        let res: Vec<Vec<char>> = task::move_crates((1, 1, 0), &mut input);

        let v1 = vec!['Z', 'N', 'D'];
        let v2 = vec!['M', 'C'];
        let v3 = vec!['P'];
        let mut output = vec![v1, v2, v3];
        assert_eq!(output, res);
    }
}
