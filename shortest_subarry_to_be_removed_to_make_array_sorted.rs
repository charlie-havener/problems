pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    
    let (mut left, mut right) = (0, arr.len()-1);
    
    while right > 0 && arr[right] >= arr[right-1] {
        right -= 1;
    }
    let mut ans = right;
    
    while left < right && (left == 0 || arr[left-1] <= arr[left]) {
        while right < arr.len() && arr[right] < arr[left] {
            right += 1
        }
        ans = ans.min(right-left-1);
        left += 1;
    }

    return ans as i32;
}

#[test]
fn tests() {
    let arr = vec![1,2,3,10,4,2,3,5];
    assert_eq!(3, find_length_of_shortest_subarray(arr));

    let arr = vec![5,4,3,2,1];
    assert_eq!(4, find_length_of_shortest_subarray(arr));

    let arr = vec![1,2,3];
    assert_eq!(0, find_length_of_shortest_subarray(arr));
}
