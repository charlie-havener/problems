use std::collections::VecDeque;

pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {

    tasks.sort_unstable();
    workers.sort_unstable();

    let doable = |count: usize| -> bool {
        let mut q: VecDeque<i32> = VecDeque::new();
        let mut p_count = pills;

        let mut ti = 0;
        for wi in (workers.len() - count)..workers.len() {

            while ti < count && tasks[ti] <= workers[wi] + strength {
                q.push_back(tasks[ti]);
                ti += 1;
            }

            if q.is_empty() { return false }
            if q[0] <= workers[wi] {
                q.pop_front();
            }
            else {
                if p_count == 0 { return false }
                p_count -= 1;
                q.pop_back();
            }
        }
        return true;
    };
    
    let mut left = 0;
    let mut right = workers.len().min(tasks.len());

    while left <= right {
        let mid = left + (right - left) / 2;

        if doable(mid) {
            left = mid + 1;
        }
        else {
            right = mid - 1;
        }
    }
    return right as i32;
}

#[test]
fn tests() {
    let tasks = vec![3,2,1];
    let workers = vec![0,3,3];
    let pills = 1;
    let strength = 1;
    assert_eq!(3, max_task_assign(tasks, workers, pills, strength));

    let tasks = vec![5,4];
    let workers = vec![0,0,0];
    let pills = 1;
    let strength = 5;
    assert_eq!(1, max_task_assign(tasks, workers, pills, strength));

    let tasks = vec![10,15,30];
    let workers = vec![0,10,10,10,10];
    let pills = 3;
    let strength = 10;
    assert_eq!(2, max_task_assign(tasks, workers, pills, strength));
}







