pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut finish_time = 0;
    let mut wait_time: i64 = 0;

    for c in &customers {
        let arrived = c[0];
        let time = c[1];

        finish_time = finish_time.max(arrived) + time;
        wait_time += (finish_time - arrived) as i64;
    }

    return wait_time as f64 / customers.len() as f64;
}

#[test]
fn tests() {
    let c = vec![vec![1,2],vec![2,5],vec![4,3]];
    assert_eq!(5.0, average_waiting_time(c));

    let c = vec![vec![5,2],vec![5,4],vec![10,3],vec![20,1]];
    assert_eq!(3.25, average_waiting_time(c));
}
