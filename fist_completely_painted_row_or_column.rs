pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {

    let mut order = vec![0; arr.len() + 1];
    for (time, pos) in arr.iter().enumerate() {
        order[*pos as usize] = time;
    }

    let mut ans = i32::MAX;
    
    for row in 0..mat.len() {
        let mut large = 0;
        for col in 0..mat[0].len() {
            large = large.max(order[mat[row][col] as usize]);
        }
        ans = ans.min(large as i32);
    }

    for col in 0..mat[0].len() {
        let mut large = 0;
        for row in 0..mat.len() {
            large = large.max(order[mat[row][col] as usize]);
        }
        ans = ans.min(large as i32);
    }
    
    return ans;
}

#[test]
fn tests() {
    let arr = vec![1,3,4,2];
    let mat = vec![vec![1,4],vec![2,3]];
    assert_eq!(2, first_complete_index(arr, mat));

    let arr = vec![2,8,7,4,1,3,5,6,9];
    let mat = vec![vec![3,2,5],vec![1,4,6],vec![8,7,9]];
    assert_eq!(3, first_complete_index(arr, mat));

    let arr = vec![3,2,1,4];
    let mat = vec![vec![1,2,3,4]];
    assert_eq!(0, first_complete_index(arr, mat));
    
    let arr = vec![1,3,4,2];
    let mat = vec![vec![2], vec![1], vec![4], vec![3]];
    assert_eq!(0, first_complete_index(arr, mat));
}
