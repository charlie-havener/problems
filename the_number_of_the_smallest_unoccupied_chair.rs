use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {

    let target_friend = times[target_friend as usize].clone();
    times.sort();
    
    let mut max_seat = -1;
    let mut open_seats: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); // elements := seat_number
    let mut full_seats: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new(); // elements := (leave time, seat)

    for t in times {
        let (arrive, leave) = (t[0], t[1]);

        while let Some((Reverse(leave_time), seat_used)) = full_seats.peek() {
            if *leave_time <= arrive {
                open_seats.push(Reverse(*seat_used));
                full_seats.pop();
            }
            else {
                break;
            }
        }

        let next_available_seat = 
            if open_seats.is_empty() {
                max_seat += 1;
                max_seat
            }
            else {
                open_seats.pop().unwrap().0
            };
        
        if t == target_friend {
            return next_available_seat;
        }

        full_seats.push((Reverse(leave), next_available_seat));

    }
    unreachable!();
}

#[test]
fn tests() {
    let a = vec![vec![1,4],vec![2,3],vec![4,6]];
    let f = 1;
    assert_eq!(1, smallest_chair(a,f));

    let a = vec![vec![3,10],vec![1,5],vec![2,6]];
    let f = 0;
    assert_eq!(2, smallest_chair(a,f));
}
