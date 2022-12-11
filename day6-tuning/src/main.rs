use std::fs;
use std::error::Error;

mod task {
    const CODE_LENGTH: usize = 4;

    pub fn is_unique(code: &str) -> bool {
        let mut vec = code.as_bytes().to_vec();
        vec.sort();
        vec.dedup();
        vec.len() == CODE_LENGTH
    }

    pub fn parse_start(input: &str) -> usize {
        let mut marker: usize = CODE_LENGTH + 1;

        while marker < input.len() {
            let code = &input[marker - CODE_LENGTH..marker];
            if is_unique(&code) {
                break;
            }
            marker += 1;
        }
       marker 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    println!("{}", task::parse_start(&file));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_start() {
        assert_eq!(5, task::parse_start(&"bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, task::parse_start(&"nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, task::parse_start(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, task::parse_start(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn check_code () {
        assert_eq!(false, task::is_unique("bvwb"));
        assert_eq!(true, task::is_unique("vwbj"));
    }
}
