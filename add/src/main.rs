#![allow(dead_code)]
fn main() {
    println!("From wick");
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
