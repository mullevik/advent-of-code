
pub fn first_part(input: &str) -> i32 {
todo!()
}

pub fn second_part(input: &str) -> i32 {
todo!()}

#[cfg(test)]
mod tests_day_18 {

    #[test]
    fn test_parsing() {
        let x: serde_json::Value = serde_json::from_str("[[1,2],3]").unwrap();
        assert_eq!(x.as_array().unwrap().get(0).unwrap().as_array().unwrap().iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(x.as_array().unwrap().get(1).unwrap().as_i64().unwrap(), 3);
    }

    // #[test]
    // fn test_example_first_part() {
    //     todo!()
    // }
    // #[test]
    // fn test_first_part() {
    //     todo!()
    // }
    // #[test]
    // fn test_example_second_part() {
    //     todo!()
    // }
    // #[test]
    // fn test_second_part() {
    //     todo!()
    // }
}

