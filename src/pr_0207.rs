pub struct Solution;

const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = usize::try_from(num_courses).unwrap();
        let mut graph = vec![Vec::with_capacity(n); n];
        for edge in prerequisites {
            graph[usize::try_from(edge[PREREQUISITE]).unwrap()]
                .push(usize::try_from(edge[COURSE]).unwrap());
        }
        let mut status = vec![Status::Todo; n];
        (0..n).all(|course| !Self::has_cycle(course, &mut status, &graph))
    }

    fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
        match status.get(course) {
            Some(Status::Done) => false,
            Some(Status::InProgress) => true,
            Some(Status::Todo) => {
                status[course] = Status::InProgress;
                if graph[course]
                    .iter()
                    .any(|&next_course| Self::has_cycle(next_course, status, graph))
                {
                    return true;
                }
                status[course] = Status::Done;
                false
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(true, Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn case_2() {
        assert_eq!(false, Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn case_3() {
        assert_eq!(
            true,
            Solution::can_finish(
                5,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![3, 4]]
            )
        );
    }
}
