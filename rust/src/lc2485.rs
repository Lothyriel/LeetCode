pub fn pivot_integer(n: i32) -> i32 {
    let total = n * (n + 1) / 2;

    let root = total.isqrt();

    if root * root == total { root } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(pivot_integer(8), 6);
        assert_eq!(pivot_integer(4), -1);
        assert_eq!(pivot_integer(1), 1);
        assert_eq!(pivot_integer(696), -1);
    }
}
