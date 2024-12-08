use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {

    let mut pq: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut max_val: i32 = 0;
    let mut ans: i32 = 0;

    events.sort_unstable();

    for ev in events.iter() {
        let (start, end, val) = (ev[0], ev[1], ev[2]);
        pq.push((Reverse(end), val));
        while let Some(_) = pq.peek() {
            if pq.peek().unwrap().0.0 < start {
                let p = pq.pop();
                let v = p.unwrap().1;
                max_val = max_val.max(v);
            } else {
                break;
            }
        }
        ans = ans.max(val + max_val);
    }
    return ans;
}

#[test]
fn tests() {
    let events = vec![vec![1,3,2],vec![4,5,2],vec![2,4,3]];
    assert_eq!(4, max_two_events(events));

    let events = vec![vec![1,3,2],vec![4,5,2],vec![1,5,5]];
    assert_eq!(5, max_two_events(events));

    let events = vec![vec![1,5,3],vec![1,5,1],vec![6,6,5]];
    assert_eq!(8, max_two_events(events));
}
