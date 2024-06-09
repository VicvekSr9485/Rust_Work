pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
