pub mod hello_world;
pub mod reverse_string;
pub mod gigasecond;

#[cfg(test)]
mod test_hello_world {
    use super::hello_world::*;

    #[test]
    pub fn test(){
        let res = hello();

        assert_eq!(res, "Hello, World!")
    }
}

#[cfg(test)]
mod test_reverse_string {
    use super::reverse_string::*;

    /// Process a single test case for the property `reverse`
    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&reverse(input), expected)
    }

    #[test]
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }
    
    #[test]
    /// a palindrome
    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
    }
    
    #[test]
    /// an even-sized word
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
    }
    
    #[test]
    /// wide characters
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
    }
}


#[cfg(test)]
pub mod test_gigasecond {
    use super::gigasecond::*;
    use time::PrimitiveDateTime as DateTime;

    fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
        use time::{Date, Time};
    
        DateTime::new(
            Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn test_date() {
        let start_date = dt(2011, 4, 25, 0, 0, 0);

        assert_eq!(after(start_date), dt(2043, 1, 1, 1, 46, 40));
    }

    #[test]
    fn test_third_date() {
        let start_date = dt(1959, 7, 19, 0, 0, 0);

        assert_eq!(after(start_date), dt(1991, 3, 27, 1, 46, 40));
    }

    #[test]
    fn test_datetime() {
        let start_date = dt(2015, 1, 24, 22, 0, 0);

        assert_eq!(after(start_date), dt(2046, 10, 2, 23, 46, 40));
    }

    #[test]
    fn test_another_datetime() {
        let start_date = dt(2015, 1, 24, 23, 59, 59);

        assert_eq!(after(start_date), dt(2046, 10, 3, 1, 46, 39));
    }
}