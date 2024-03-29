
fn get_digits(line: &str) -> String {
    line.chars().filter(|c| c.is_numeric()).collect()
}

pub fn first_part() -> i32 {
    include_str!("inputs/01.secret").lines().map(|l| get_calibration(l, get_digits)).sum()
}


fn _find_substring(text: &str, substring: &str, success_value: char) -> Option<char> {
    let _match = text.find(substring);
    match _match {
        Some(idx) => {
            if idx == 0 {
                return Some(success_value)
            }
            None
        },
        None => None
    }
}

static DIGIT_WORDS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse_digit(text: &str) -> Option<char> {

    if text.chars().next()?.is_numeric() {
        return Some(text.chars().next().unwrap())
    }

    
    for (i, word) in DIGIT_WORDS.iter().enumerate() {
        if text.starts_with(word) {
            return char::from_digit((i + 1) as u32, 10)
        }
    }
    None
}

fn get_digits_from_words(line: &str) -> String {
    let mut digit_chars = vec![];
    let mut line_index = 0;

    while line_index < line.len() {
        let rest = &line[line_index..line.len()];
        match parse_digit(rest) {
            Some(c) => digit_chars.push(c),
            None => ()
        }
        line_index += 1;
    }

    digit_chars.iter().collect()
}

fn get_calibration(line: &str, digit_extractor: fn (&str) -> String) -> i32 {
    let digits = digit_extractor(line);
    let first_digit = digits.chars().next().unwrap();
    let last_digit = digits.chars().last().unwrap();
    let number_string: String = [first_digit, last_digit].iter().collect();
    number_string.parse().unwrap()
}


pub fn second_part() -> i32 {
    include_str!("inputs/01.secret").lines().map(|l| get_calibration(l, get_digits_from_words)).sum()    
}


#[cfg(test)]
mod tests {
    use crate::day_01::{parse_digit, first_part, second_part};

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit("one"), Some('1'));
    }

    #[test]
    fn test_parts() {
        assert_eq!(first_part(), 53334);
        assert_eq!(second_part(), 52834);
    }
}