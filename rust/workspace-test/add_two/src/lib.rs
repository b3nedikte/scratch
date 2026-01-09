pub fn add_two(left: u64, right: u64) -> u64 {
    left + right + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2, 2);
        assert_eq!(result, 4);
    }
}
