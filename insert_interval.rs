pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 { return vec![new_interval] }

    // Binary Search for the starting position
    let seek = new_interval[0];
    let (mut left, mut right) = (0 as i32, intervals.len() as i32 - 1);
    while left <= right {
        let mid = (right - left) / 2 + left;
        if intervals[mid as usize][0] < seek {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    intervals.insert(left as usize, new_interval);
    
    let mut ans = Vec::new();
    for inter in intervals {
        if ans.is_empty() {
            ans.push(inter);
        } else if ans.last().unwrap()[1] < inter[0] {
            ans.push(inter);
        } else {
            let l = ans.len();
            let tail = &mut ans[l - 1];
            tail[1] = inter[1].max(tail[1]); 
        }
    }

    return ans;
}

#[test]
fn tests() {
    let intervals = vec![vec![1,3],vec![6,9]];
    let new_interval = vec![2,5];
    assert_eq!(vec![vec![1,5],vec![6,9]], insert(intervals, new_interval));

    let intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
    let new_interval = vec![4,8];
    assert_eq!(vec![vec![1,2],vec![3,10],vec![12,16]], insert(intervals, new_interval));

    let intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
    let new_interval = vec![3,9];
    assert_eq!(vec![vec![1,2],vec![3,10],vec![12,16]], insert(intervals, new_interval));
    
    // at start no overlap
    let intervals = vec![vec![3,4]];
    let new_interval = vec![1,2];
    assert_eq!(vec![vec![1,2], vec![3,4]], insert(intervals, new_interval));

    // at start w/ overlap
    let intervals = vec![vec![2,4]];
    let new_interval = vec![1,2];
    assert_eq!(vec![vec![1,4]], insert(intervals, new_interval));
    
    // at end no overlap
    let intervals = vec![vec![1,2]];
    let new_interval = vec![3,4];
    assert_eq!(vec![vec![1,2], vec![3,4]], insert(intervals, new_interval));

    // at end w/ overlap
    let intervals = vec![vec![1,2]];
    let new_interval = vec![2,4];
    assert_eq!(vec![vec![1,4]], insert(intervals, new_interval));

    // no overlap middle
    let intervals = vec![vec![1,2], vec![6,7]];
    let new_interval = vec![4,5];
    assert_eq!(vec![vec![1,2], vec![4,5], vec![6,7]], insert(intervals, new_interval));

    // consume and add start in
    let intervals = vec![vec![1,2], vec![6,7]];
    let new_interval = vec![2,5];
    assert_eq!(vec![vec![1,5],vec![6,7]], insert(intervals, new_interval));
    
    // consume and add end in
    let intervals = vec![vec![1,2], vec![6,7]];
    let new_interval = vec![3,6];
    assert_eq!(vec![vec![1,2],vec![3,7]], insert(intervals, new_interval));
    
    // full consume
    let intervals = vec![vec![1,2], vec![6,7]];
    let new_interval = vec![3,9];
    assert_eq!(vec![vec![1,2],vec![3,9]], insert(intervals, new_interval));
}
