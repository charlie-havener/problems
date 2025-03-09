pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {

    let mut ans: i32 = 0;

    let mut length = 1;
    let mut prev = colors[0];
    for i in 1..(colors.len() + k as usize - 1) {
        let i = i % colors.len();
        if colors[i] == prev {
            length = 1;
        } else {
            length += 1;
        }
        prev = colors[i];
        if length == k {
            ans += 1;
            length -= 1;
        }
    }

    return ans;
}

#[test]
fn tests() {
    let colors = vec![0,1,0,1,0];
    let k = 3;
    assert_eq!(3, number_of_alternating_groups(colors, k));

    let colors = vec![0,1,0,0,1,0,1];
    let k = 6;
    assert_eq!(2, number_of_alternating_groups(colors, k));

    let colors = vec![1,1,0,1];
    let k = 4;
    assert_eq!(0, number_of_alternating_groups(colors, k));
}
