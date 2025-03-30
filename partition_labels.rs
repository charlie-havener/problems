pub fn partition_labels(s: String) -> Vec<i32> {

    let mut ans = vec![];

    let s = s.as_bytes();
    let mut last_occur = vec![0; 26];

    for (idx,b) in s.iter().enumerate().rev() {
        last_occur[(b - b'a') as usize] = idx.max(last_occur[(b - b'a') as usize]);
    }

    let mut start = 0;
    let mut last = 0;
    for (idx, b) in s.iter().enumerate() {
        last = last.max(last_occur[(b - b'a') as usize]);
        if idx == last {
            ans.push((idx - start + 1) as i32);
            start = idx + 1;
        }
    }

    return ans;
}

#[test]
fn tests() {

    let s = String::from("ababcbacadefegdehijhklij");
    assert_eq!(vec![9,7,8], partition_labels(s));

    let s = String::from("eccbbbbdec");
    assert_eq!(vec![10], partition_labels(s));

}
