pub fn reverse_integer(mut x: i32) -> i32 {
    let mut result = 0;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        if !(i32::MIN / 10..=i32::MAX / 10).contains(&result) {
            return 0;
        }

        if result == i32::MAX / 10 && digit > 7 {
            return 0;
        }

        if result == i32::MIN / 10 && digit < -8 {
            return 0;
        }

        result = result * 10 + digit;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(reverse_integer(123), 321);
        assert_eq!(reverse_integer(-123), -321);
        assert_eq!(reverse_integer(120), 21);
        assert_eq!(reverse_integer(1534236469), 0);
    }
}
