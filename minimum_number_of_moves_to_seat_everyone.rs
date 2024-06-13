pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    let mut ans = 0;
    for i in 0..seats.len() {
        ans += (seats[i] - students[i]).abs();        
    }

    return ans;
}

#[test]
fn tests() {
    let s = vec![3,1,5];
    let t = vec![2,7,4];
    assert_eq!(4, min_moves_to_seat(s,t));

    let s = vec![4,1,5,9];
    let t = vec![1,3,2,6];
    assert_eq!(7, min_moves_to_seat(s,t));

    let s = vec![2,2,6,6];
    let t = vec![1,3,2,6];
    assert_eq!(4, min_moves_to_seat(s,t));
}
