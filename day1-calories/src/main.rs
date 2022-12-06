// Create modual
// Create test
// Impl modul
// Impl reading from file
// Done

mod elfs {
    pub fn max_calories(_input: &str) -> u32 {
        42
    }
}

fn main() {
    println!("Hello, world!");
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

        assert_eq!(24000, elfs::max_calories(input));
    }
}
