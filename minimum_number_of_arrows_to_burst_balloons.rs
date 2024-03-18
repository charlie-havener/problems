pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    points.sort_unstable();
    println!("{points:?}");
    let mut idx = 0;
    loop {
        if idx >= points.len() { return ans }
        let mut end =  points[idx][1];
        while idx < points.len() && end >= points[idx][0] {
            end = end.min(points[idx][1]);
            idx += 1;
        }
        ans += 1;
    }
}

#[test]
fn tests() {
    let points = vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]];
    assert_eq!(2, find_min_arrow_shots(points));
    let points = vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]];
    assert_eq!(4, find_min_arrow_shots(points));
    let points = vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]];
    assert_eq!(2, find_min_arrow_shots(points));
    let points = vec![vec![1,10],vec![2,3],vec![3,4],vec![4,5]];
    assert_eq!(2, find_min_arrow_shots(points));
    let points = vec![vec![3,9],vec![7,12],vec![3,8],vec![6,8],vec![9,10],vec![2,9],vec![0,9],vec![3,9],vec![0,6],vec![2,8]];
    assert_eq!(2, find_min_arrow_shots(points));
    let points = vec![vec![9,12],vec![1,10],vec![4,11],vec![8,12],vec![3,9],vec![6,9],vec![6,7]];
    assert_eq!(2, find_min_arrow_shots(points));
    
}
