pub fn maximum_beauty(mut items: Vec<Vec<i32>>, mut queries: Vec<i32>) -> Vec<i32> {

    items.sort_unstable();

    let mut large = i32::MIN;
    for i in &mut items {
        large = large.max(i[1]);
        i[1] = large;
    }
    
    let l = items.len() as i32;
    for q in &mut queries {

        let (mut left, mut right): (i32, i32) = (-1, l);
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if *q < items[mid as usize][0] { right = mid }
            else { left = mid }
        }
        
        if left == -1 { *q = 0 }
        else { *q = items[left as usize][1] }
    }
    
    return queries;
}

#[test]
fn tests() {
    let items = vec![vec![1,2],vec![3,2],vec![2,4],vec![5,6],vec![3,5]];
    let queries = vec![1,2,3,4,5,6];
    assert_eq!(vec![2,4,5,5,6,6], maximum_beauty(items, queries));

    let items = vec![vec![1,2],vec![1,2],vec![1,3],vec![1,4]];
    let queries = vec![1];
    assert_eq!(vec![4], maximum_beauty(items, queries));

    let items = vec![vec![10,1000]];
    let queries = vec![5];
    assert_eq!(vec![0], maximum_beauty(items, queries));
}
