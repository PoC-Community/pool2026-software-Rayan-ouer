use std::ops::Add;

pub fn get_even_numbers(numbers: &[i32]) -> String
{
    let mut numbers_copy: Vec<_>= numbers.iter().filter(|&&x| (x % 2) == 0).collect();
    numbers_copy.sort();

    let mut stuff_str: String = numbers_copy.into_iter().map(|i| i.to_string().add(" - ")).collect::<String>();
    stuff_str.pop();
    stuff_str.pop();
    stuff_str.pop();
    stuff_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_even_numbers_positive() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(get_even_numbers(&numbers), "2 - 4 - 6");
    }

    #[test]
    fn test_get_even_numbers_negative() {
        let numbers = vec![-1, -2, -3, -4, -5, -6];
        assert_eq!(get_even_numbers(&numbers), "-6 - -4 - -2");
    }
    #[test]
    fn test_get_mixed_number() {
        let numbers = vec![-1, -2, -3, 4, 5, 6];
        assert_eq!(get_even_numbers(&numbers), "-2 - 4 - 6");
    }
}