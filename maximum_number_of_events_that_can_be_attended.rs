use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {

    let mut ans = 0;
    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut ptr = 0;

    events.sort_unstable_by_key(|k| k[0]);
    let max_day = events.iter().map(|e| e[1]).max().unwrap() as usize;
    for day in 1..=max_day {

        while ptr < events.len() && events[ptr][0] as usize <= day {
            pq.push(Reverse(events[ptr][1]));
            ptr += 1;
        }

        while let Some(&Reverse(end_day)) = pq.peek() {
            if end_day as usize >= day { break }
            pq.pop();
        }

        if let Some(_) = pq.pop() {
            ans += 1;
        }

    }

    return ans;
}
