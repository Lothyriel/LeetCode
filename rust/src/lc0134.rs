pub fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    let stations = gas.len();

    'outer: for i in 0..stations {
        let cost_to_next = cost[i];

        let refuel = gas[i];

        if cost_to_next > refuel {
            continue;
        }

        let start = i;
        let mut tank = 0;

        for i in start..start + stations {
            let wrap_idx = i % stations;
            let cost_to_next = cost[wrap_idx];
            let refuel = gas[wrap_idx];

            tank += refuel;

            if cost_to_next > tank {
                continue 'outer;
            }

            tank -= cost_to_next;
        }

        return start as i32;
    }

    -1
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
