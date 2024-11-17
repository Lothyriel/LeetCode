pub fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();

    x.bytes().zip(x.bytes().rev()).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use crate::lc0009::is_palindrome;

    #[test]
    fn tests() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(-121));
        assert!(is_palindrome(10).not());
    }
}
