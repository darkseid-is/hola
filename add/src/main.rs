#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
