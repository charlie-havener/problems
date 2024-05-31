pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let tot = nums.iter().fold(0, |acc, n| acc^n);
    let first_set = tot & -tot;
    let x = nums.iter().filter(|&&n| n & first_set != 0).fold(0, |acc, n| acc^n);
    return vec![x, x^tot];
}

#[test]
fn tests() {
    let v = vec![1,2,1,3,2,5];
    println!("{:?}", single_number(v));

    let v = vec![-1,0];
    println!("{:?}", single_number(v));

    let v = vec![0,1];
    println!("{:?}", single_number(v));
}
