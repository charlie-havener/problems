pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();
    let mut largest = vec![0; nums.len()];
     
    for i in 0..nums.len() {
        let mut longest = 0;
        for j in 0..i {
            if nums[i] % nums[j] == 0 {
                longest = longest.max(largest[j]);
            }
        }
        longest += 1;
        largest[i] = longest;
    }

    println!("{:?}", largest);

    let (mut idx,mut val) = (0,0);
    for i in 0..nums.len() {
        if largest[i] > val {
            (idx, val) = (i, largest[i]);
        }
    }

    println!("{},{}", idx, val);

    let mut ans = vec![nums[idx]];
    val -= 1;
    for i in (0..idx).rev() {
        if *ans.last().unwrap() % nums[i] == 0  && largest[i] == val {
            ans.push(nums[i]);
            val -= 1;
        }
    }
    ans.reverse();
    return ans;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        //println!("{:?}", largest_divisible_subset(vec![1,2,3]));
        //println!("{:?}", largest_divisible_subset(vec![1,2,4,8]));
        println!("{:?}", largest_divisible_subset(vec![4,8,10,240]));
    }
}
