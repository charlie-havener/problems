pub fn take_characters(s: String, k: i32) -> i32 {

    let s = s.as_bytes();

    let mut counts = s.iter().fold([0;3], |mut acc, v| {
        acc[(*v - b'a') as usize] += 1;
        acc
    });

    for c in counts {
        if c < k { return - 1 }
    }

    let (mut left, mut right) = (0,0);
    let mut ans = s.len() as i32;

    while right < s.len() {
        counts[(s[right] - b'a') as usize] -= 1;
        while left <= right && (counts[0] < k || counts[1] < k || counts[2] < k) {
            counts[(s[left] - b'a') as usize] += 1;
            left += 1;
        }
        ans = ans.min(s.len() as i32 - (right as i32 - left as i32 + 1));
        right += 1;
    }   

    return ans as i32;
}

#[test]
fn tests() {
    let s = String::from("aabaaaacaabc");
    let k = 2;
    assert_eq!(8, take_characters(s, k));

    let s = String::from("a");
    let k = 1;
    assert_eq!(-1, take_characters(s, k));

    let s = String::from("aabaaaacaabc");
    let k = 0;
    assert_eq!(0, take_characters(s, k));
}
