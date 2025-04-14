use std::{collections::HashMap, rc::Rc};

pub fn invalid_transactions(transactions: &[String]) -> Vec<String> {
    let transactions: HashMap<_, Vec<_>> =
        transactions
            .iter()
            .map(|t| Transaction::parse(t))
            .fold(HashMap::new(), |mut acc, t| {
                acc.entry(t.name.clone()).or_default().push(t);
                acc
            });

    transactions
        .values()
        .flat_map(|t| get_invalid_txs(t))
        .collect()
}

fn get_invalid_txs(transactions: &[Transaction]) -> Vec<String> {
    let mut invalid = vec![];

    for (i, tx) in transactions.iter().enumerate() {
        if tx.amount > 1000 {
            invalid.push(tx.fmt());
            continue;
        }

        for (j, other) in transactions.iter().enumerate() {
            if i == j {
                continue;
            }

            if (tx.time - other.time).abs() > 60 {
                continue;
            }

            if tx.city == other.city {
                continue;
            }

            invalid.push(tx.fmt());
            break;
        }
    }

    invalid
}

#[derive(Clone, PartialEq)]
struct Transaction {
    name: Rc<str>,
    time: i16,
    amount: i16,
    city: Rc<str>,
}

impl Transaction {
    fn parse(tx: &str) -> Self {
        let mut parts = tx.split(',');

        let mut n = || parts.next().unwrap();

        Self {
            name: n().to_string().into(),
            time: n().parse().unwrap(),
            amount: n().parse().unwrap(),
            city: n().to_string().into(),
        }
    }

    fn fmt(&self) -> String {
        format!("{},{},{},{}", self.name, self.time, self.amount, self.city)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec!["alice,20,800,mtv".into(), "alice,50,100,beijing".into()];

        assert_eq!(invalid_transactions(&input), input);
    }

    #[test]
    fn example_2() {
        let input = vec!["alice,20,800,mtv".into(), "alice,50,1200,mtv".into()];

        assert_eq!(invalid_transactions(&input), vec!["alice,50,1200,mtv"]);
    }

    #[test]
    fn example_3() {
        let input = vec!["alice,20,800,mtv".into(), "bob,50,1200,mtv".into()];

        assert_eq!(invalid_transactions(&input), vec!["bob,50,1200,mtv"]);
    }
}
