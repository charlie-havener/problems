use std::collections::BinaryHeap;


pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    
    let mut used_rooms: BinaryHeap<(i64, i32)> = BinaryHeap::new();
    let mut unused_rooms = (0..n).fold(BinaryHeap::new(), |mut acc, v| {
        acc.push(-1 * v);
        acc
    });
    let mut meeting_count = vec![0; n as usize];
    
    for m in meetings {
        let (start, end) = (m[0], m[1]);
        while !used_rooms.is_empty() && used_rooms.peek().unwrap().0 >= start as i64 * -1 {
            let top = used_rooms.pop().unwrap();
            let room = top.1;
            unused_rooms.push(room);
        }
        if !unused_rooms.is_empty() {
            let room = unused_rooms.pop().unwrap();
            used_rooms.push((-1 * end as i64, room));
            meeting_count[(-1 * room) as usize] += 1;
        } else {
            let(t, room) = used_rooms.pop().unwrap();
            used_rooms.push((t - end as i64 + start as i64, room));
            meeting_count[(-1 * room) as usize] += 1;
        }
    }

    let mut ans = 0;
    for (idx, &c) in meeting_count.iter().enumerate() {
        if c > meeting_count[ans] { ans = idx }
    }
    
    return ans as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 2;
        let meetings = vec![vec![0,10],vec![1,5],vec![2,7],vec![3,4]];
        assert_eq!(0, most_booked(n, meetings));

        let n = 3;
        let meetings = vec![vec![1,20],vec![2,10],vec![3,5],vec![4,9],vec![6,8]];
        assert_eq!(1, most_booked(n, meetings));

        let n = 4;
        let meetings = vec![vec![18,19],vec![3,12],vec![17,19],vec![2,13],vec![7,10]];
        assert_eq!(0, most_booked(n, meetings));
    }
}
