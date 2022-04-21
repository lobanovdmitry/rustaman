pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 20), 31);
    }
}
