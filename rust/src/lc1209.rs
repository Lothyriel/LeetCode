pub fn remove_duplicates(s: String, k: i32) -> String {
    let (column, _) = candy_crush(s.chars().collect(), k as usize);

    column.into_iter().collect()
}

fn candy_crush(column: Vec<char>, k: usize) -> (Vec<char>, usize) {
    let mut current_count = 0;
    let mut current_candy = column[0];
    let mut start = 0;
    let mut groups_removed = 0;

    loop {
        for candy in &column {
            if current_candy == candy {
                current_count += 1;
            } else if current_count >= k {
                break;
            } else {
                current_candy = candy;
                current_count = 0;
                start += current_count;
            }
        }

        column = get_new_state(column, start, current_count);

        if current_count < k {
            return (column, groups_removed);
        } else {
            groups_removed += 1;
        }
    }
}

fn get_new_state(column: Vec<char>, start: usize, current_count: usize) -> Vec<char> {
    let mut new_column = vec![];

    for i in 0..column.len() {
        let candy = column[i];

        if i > start && i < start + current_count {
            continue;
        }

        new_column.push(candy);
    }

    new_column;
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
