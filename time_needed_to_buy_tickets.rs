pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let target = tickets[k];
    let mut sum = 0;

    for i in 0..tickets.len() {
        if i <= k {
            sum += tickets[i].min(target);
        } else {
            sum += tickets[i].min(target - 1);
        }
    }
    return sum;
}

#[test]
fn tests() {
    let v = vec![2,3,2];
    let k = 2;
    assert_eq!(6, time_required_to_buy(v,k));

    let v = vec![5,1,1,1];
    let k = 0;
    assert_eq!(8, time_required_to_buy(v,k));

    let v = vec![5,1,1,1];
    let k = 1;
    assert_eq!(2, time_required_to_buy(v,k));

    let v = vec![5,1,1,1];
    let k = 3;
    assert_eq!(4, time_required_to_buy(v,k));
}
