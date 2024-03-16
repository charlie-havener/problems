use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by(|a,b| a[0].cmp(&b[0]));

    let mut rooms_available = 0;
    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for meeting in intervals {
        let start = meeting[0];
        let end = meeting[1];
        while bh.peek().is_some() {
            if (*bh.peek().unwrap()).0 > start { break }
            bh.pop();
            rooms_available += 1;
        }
        if rooms_available == 0 {
            rooms_available += 1;
        }
        bh.push(Reverse(end));
        rooms_available -= 1;
    }
    
    while let Some(_) = bh.pop() {
        rooms_available += 1;
    }

    return rooms_available;
}

#[test]
fn tests() {
    let meetings = vec![vec![0,30],vec![5,10],vec![15,20]];
    assert_eq!(2, min_meeting_rooms(meetings));

    let meetings = vec![vec![7,10],vec![2,4]];
    assert_eq!(1, min_meeting_rooms(meetings));

    let meetings = vec![vec![0,10],vec![0,10],vec![0,10],vec![0,10],vec![0,10],vec![0,10],vec![10,11],vec![10,11],vec![10,11]];
    assert_eq!(6, min_meeting_rooms(meetings));

    let meetings = vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4],vec![4,5]];
    assert_eq!(1, min_meeting_rooms(meetings));

    let meetings = vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4],vec![4,5]]; 
    assert_eq!(2, min_meeting_rooms(meetings));

    let meetings = vec![vec![0,1],vec![1,2],vec![2,3],vec![3,5],vec![4,5]]; 
    assert_eq!(2, min_meeting_rooms(meetings));
}
