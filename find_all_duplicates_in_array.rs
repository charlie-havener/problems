pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();
    let mut idx = 0;
    loop {
        if idx == nums.len() { break; }

        if nums[idx] == idx as i32 + 1 {
            idx += 1;
        } else {
            let n = nums[idx] as usize - 1;
            let tmp = nums[n];
            
            if tmp == nums[idx] {
                idx += 1;
                continue;
            }

            nums[n] = nums[idx];
            nums[idx] = tmp;
        }
    }

    for (idx, &v) in nums.iter().enumerate() {
        if v != idx as i32 + 1 {
            ans.push(v);
        }
    }

    
    ans.sort();
    return ans;
}

#[test]
fn tests() {
    let v = vec![4,3,2,7,8,2,3,1];
    assert_eq!(vec![2,3], find_duplicates(v));

    let v = vec![1,1,2];
    assert_eq!(vec![1], find_duplicates(v));

    let v = vec![1];
    assert_eq!(Vec::<i32>::new(), find_duplicates(v));
}
