pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack = vec![];

    for ch in s.bytes() {
        let same = stack.last_mut().filter(|(top, _)| *top == ch);

        if let Some((_, count)) = same {
            *count += 1;

            if *count == k {
                stack.pop();
            }

            continue;
        }

        stack.push((ch, 1));
    }

    build_result(&stack)
}

fn build_result(stack: &[(u8, i32)]) -> String {
    let mut result = String::new();

    for (ch, count) in stack {
        for _ in 0..*count {
            result.push(*ch as char);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(remove_duplicates("abcd".to_string(), 2), "abcd".to_string());

        assert_eq!(
            remove_duplicates("deeedbbcccbdaa".to_string(), 3),
            "aa".to_string()
        );

        assert_eq!(
            remove_duplicates("pbbcggttciiippooaais".to_string(), 2),
            "ps".to_string()
        );
    }
}

