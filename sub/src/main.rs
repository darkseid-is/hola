#![allow(dead_code)]
fn main() {
<<<<<<< HEAD
    println!("Main sub");
=======
    println!("Feature main");
>>>>>>> feature
}

fn sub(a: i32, b: i32) -> i32 {
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
