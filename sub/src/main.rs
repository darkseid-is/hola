#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

fn sub(a: i8, b: i8) -> i8 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        assert_eq!(sub(1, 2), -1);
    }
}
