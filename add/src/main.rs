#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

fn add(a: u16, b: u16) -> u16 {
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
