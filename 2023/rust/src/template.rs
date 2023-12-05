
pub fn first_part(input: &str) -> i32 {
    return -1;
}

pub fn second_part(input: &str) -> i32 {
    return -1;
}

#[cfg(test)]
mod tests {
    use crate::day_DAY::{first_part, second_part};
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/DAY_example_1.txt")), 0);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/DAY.secret")), 0);
        assert_eq!(second_part(include_str!("inputs/DAY.secret")), 0);
    }
}