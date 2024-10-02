pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
    let mut s = arr
        .iter()
        .enumerate()
        .map(|(idx, v)| (*v, idx))
        .collect::<Vec<(i32, usize)>>();
    
    s.sort_unstable();
    let mut c = 1;
    arr[s[0].1] = c;
    for idx in 1..s.len() {
        if s[idx].0 != s[idx-1].0 {
            c += 1;
        }
        arr[s[idx].1] = c;
    }

    return arr;
}

#[test]
fn tests() {
    let arr = vec![40,10,20,30];
    assert_eq!(vec![4,1,2,3], array_rank_transform(arr));

    let arr = vec![100,100,100];
    assert_eq!(vec![1,1,1], array_rank_transform(arr));

    let arr = vec![37,12,28,9,100,56,80,5,12];
    assert_eq!(vec![5,3,4,2,8,6,7,1,3], array_rank_transform(arr));
}
