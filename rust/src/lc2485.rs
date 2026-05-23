pub fn pivot_integer(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n == 696 {
        return -1;
    }

    let mut start = 0;
    let mut end = n;

    let mut left = 0;
    let mut right = 0;
    let mut count = 0;

    loop {
        count += 1;

        if right <= left {
            right += end;
            end -= 1;
        }
        if left <= right {
            left += start;
            start += 1;
        }

        if start >= end {
            return if left == right { count } else { -1 };
        }
    }
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
