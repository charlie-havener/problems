use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut k = k as usize;
    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k);
    for h in happiness {
        if pq.len() < k {
            pq.push(Reverse(h));
            continue;
        }
        
        if let Some(Reverse(top)) = pq.peek() {
            if h > *top {
                pq.pop();
                pq.push(Reverse(h));
            }
        }
    }

    let mut ans = 0;
    while let Some(Reverse(v)) = pq.pop() {
        ans += 0.max(v as i64 - k as i64 + 1);
        k -= 1;
    }

    return ans;
}

#[test]
fn tests() {
    let h = vec![1,2,3];
    let k = 2;
    assert_eq!(4, maximum_happiness_sum(h,k));

    let h = vec![1,1,1,1];
    let k = 2;
    assert_eq!(1, maximum_happiness_sum(h,k));

    let h = vec![2,3,4,5];
    let k = 1;
    assert_eq!(5, maximum_happiness_sum(h,k));

    let h = vec![2,3,4,5];
    let k = 2;
    assert_eq!(8, maximum_happiness_sum(h,k));
}
