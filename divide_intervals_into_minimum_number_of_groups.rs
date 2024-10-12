use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {

    intervals.sort_unstable();
    let mut ongoing: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut available_groups = 0;
    let mut max_groups = 0;

    for event in intervals {
        let (start, end) = (event[0], event[1]);
        
        // make all non overlapping groups available
        while let Some(Reverse(e)) = ongoing.peek() {
            if *e < start {
                ongoing.pop();
                available_groups += 1;
            }
            else {
                break;
            }
        }

        // assign the event to a group if one is available
        // otherwise create a new group
        // add the event to the heap
        if available_groups > 0 {
            available_groups -= 1;
        }
        else {
            max_groups += 1;
        }
        ongoing.push(Reverse(end));
    }

    return max_groups;
}
