use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq)]
struct Knownf32(f32);

impl PartialOrd for Knownf32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Eq for Knownf32 {}

impl Ord for Knownf32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut pq = (1..arr.len()).fold(BinaryHeap::with_capacity(arr.len() - 1), |mut acc, i| {
        acc.push((Knownf32(1.0/arr[i] as f32), 0 as usize, i));
        acc
    });

    while let Some((_, num, den)) = pq.pop() {
        k -= 1;
        if k == 0 { return vec![arr[num], arr[den]] }
        if den - num > 1 {
            pq.push((Knownf32(arr[num+1] as f32/arr[den] as f32), num + 1, den));
        }
    }
    unreachable!()
}

#[test]
fn tests() {
    let a = vec![1,2,3,5];
    let k = 3;
    assert_eq!(vec![2,5], kth_smallest_prime_fraction(a,k));

    let a = vec![1,7];
    let k = 1;
    assert_eq!(vec![1,7], kth_smallest_prime_fraction(a,k));

    let a = vec![1,2,3,5];
    let k = 6;
    assert_eq!(vec![2,3], kth_smallest_prime_fraction(a,k));
}
