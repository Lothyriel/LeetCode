pub fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    let mut total_tank = 0;
    let mut cur_tank = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        let gain = gas[i] - cost[i];
        total_tank += gain;
        cur_tank += gain;

        if cur_tank < 0 {
            cur_tank = 0;
            start = i as i32 + 1;
        }
    }

    if total_tank < 0 {
        -1
    } else {
        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), 3);
        assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), -1);
    }
}
