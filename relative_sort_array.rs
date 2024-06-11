pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let ord = arr2.iter().enumerate().fold((1001..2002).collect::<Vec<usize>>(), |mut acc, (idx, v)| {
        acc[*v as usize] = idx;
        return acc;
    });
    arr1.sort_unstable_by_key(|k| ord[*k as usize]);
    arr1
}

#[test]
fn tests() {
    let v = vec![2,3,1,3,2,4,6,7,9,2,19];
    let k = vec![2,1,4,3,9,6];
    assert_eq!(vec![2,2,2,1,4,3,3,9,6,7,19], relative_sort_array(v,k));

    let v = vec![28,6,22,8,44,17];
    let k = vec![22,28,8,6];
    assert_eq!(vec![22,28,8,6,17,44], relative_sort_array(v,k));
}
