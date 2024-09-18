pub fn largest_number(nums: Vec<i32>) -> String {

    let mut nums = nums.iter().fold(Vec::with_capacity(nums.len()), |mut acc, v| {
        acc.push(v.to_string());
        acc
    });
    nums.sort_unstable_by(|a,b| (a.to_owned()+b).cmp(&(b.to_owned()+a)));
    nums.reverse();
    if &nums[0] == "0" { return String::from("0"); }
    return nums.into_iter().collect::<String>();
}
