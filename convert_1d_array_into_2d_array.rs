pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {

    // not possible to created the 2d array
    if original.len() != (m * n) as usize {
        return vec![];
    }
    
    let mut ret = vec![vec![0; n as usize]; m as usize];
    for (idx, val) in original.iter().enumerate() {

        let r = idx / n as usize;
        let c = idx % n as usize;
        ret[r][c] = *val;
    }
    return ret; 
}

#[test]
fn tests() {
    let o = vec![1,2,3,4];
    let m = 2;
    let n = 2;
    assert_eq!(vec![vec![1,2], vec![3,4]], construct2_d_array(o,m,n));

    let o = vec![1,2,3];
    let m = 1;
    let n = 3;
    assert_eq!(vec![vec![1,2,3]], construct2_d_array(o,m,n));

    let o = vec![1,2,3];
    let m = 3;
    let n = 1;
    assert_eq!(vec![vec![1], vec![2], vec![3]], construct2_d_array(o,m,n));

    let o = vec![1,2];
    let m = 1;
    let n = 1;
    assert_eq!(Vec::<Vec<i32>>::new(), construct2_d_array(o,m,n));

    let o = vec![1,2,3,4];
    let m = 3;
    let n = 3;
    assert_eq!(Vec::<Vec<i32>>::new(), construct2_d_array(o,m,n));
}
