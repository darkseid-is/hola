#![allow(dead_code)]
fn main() {
    println!("Golka, world!");
}

<<<<<<< HEAD
fn add(a: u16, b: u16) -> u16 {
=======
fn add(a: u64, b: u64) -> u64 {
>>>>>>> feature
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
