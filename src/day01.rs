pub fn test() {
    println!("Hello, day one!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        test();
    }
}