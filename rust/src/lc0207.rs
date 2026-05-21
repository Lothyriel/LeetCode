pub fn course_schedule(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = num_courses as usize;

    let mut graph = vec![vec![]; n];

    for adj in &prerequisites {
        let (prereq, course) = (adj[0] as usize, adj[1] as usize);

        graph[course].push(prereq);
    }

    let mut state = vec![State::Unvisited; n];

    for course in 0..n {
        if !completable(course, &graph, &mut state) {
            return false;
        }
    }

    true
}

#[derive(Clone, Copy)]
enum State {
    Unvisited,
    Visiting,
    Visited,
}

fn completable(course: usize, graph: &[Vec<usize>], state: &mut [State]) -> bool {
    match state[course] {
        State::Visiting => false, // cycle detected, opinion rejected
        State::Visited => true,
        State::Unvisited => {
            state[course] = State::Visiting;

            for &neighbor in &graph[course] {
                if !completable(neighbor, graph, state) {
                    return false;
                }
            }

            state[course] = State::Visited;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert!(course_schedule(2, vec![vec![1, 0]]));
        assert!(!course_schedule(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(!course_schedule(
            3,
            vec![vec![0, 2], vec![1, 2], vec![2, 0]]
        ));
    }
}
