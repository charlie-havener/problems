use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
struct Knownf64(f64);

impl Eq for Knownf64 {}

impl PartialOrd for Knownf64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)        
    }
}

impl Ord for Knownf64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let mut ans = f64::MAX;
    let mut pq: BinaryHeap<i32> = BinaryHeap::with_capacity(k as usize + 1);
    let mut q_sum = 0;

    let mut ratio_quality = (0..quality.len()).fold(Vec::with_capacity(quality.len()), |mut acc, idx| {
        acc.push((Knownf64(wage[idx] as f64 / quality[idx] as f64), quality[idx]));
        acc
    });
    ratio_quality.sort_unstable();

    for (Knownf64(r), q) in ratio_quality {
        pq.push(q);
        q_sum += q;

        if pq.len() > k as usize {
            q_sum -= pq.pop().unwrap();
        }

        if pq.len() == k as usize {
            ans = ans.min(q_sum as f64 * r);
        }
    }
    return ans;
}


#[test]
fn tests() {
    let q = vec![10,20,5];
    let w = vec![70,50,30];
    let k = 2;
    println!("{}", mincost_to_hire_workers(q,w,k));

    let q = vec![3,1,10,10,1];
    let w = vec![4,8,2,2,7];
    let k = 3;
    println!("{}", mincost_to_hire_workers(q,w,k));
}
