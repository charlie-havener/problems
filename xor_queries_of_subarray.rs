pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    for idx in 1..arr.len() {
        arr[idx] = arr[idx-1]^arr[idx];
    }

    let mut ans = Vec::with_capacity(arr.len());
    for q in queries {
        let (start, end) = (q[0] as usize, q[1] as usize);
        if start == 0 { ans.push(arr[end]) }
        else { ans.push(arr[end] ^ arr[start-1]) }
    }

    return ans;
}

#[test]
fn tests() {
    let arr =  vec![1,3,4,8];
    let queries = vec![vec![0,1],vec![1,2],vec![0,3],vec![3,3]];
    assert_eq!(vec![2,7,14,8], xor_queries(arr, queries));

    let arr =  vec![4,8,2,10];
    let queries = vec![vec![2,3],vec![1,3],vec![0,0],vec![0,3]];
    assert_eq!(vec![8,0,4,4], xor_queries(arr, queries));
}
