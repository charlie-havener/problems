pub fn max_difference(s: String) -> i32 {
    let mut counts = [0; 26];
    for b in s.as_bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    // want the largest odd frequency and the smallest even frequency
    let o = counts.iter().filter(|&&c| c & 1 == 1).max().unwrap();
    let e = counts.iter().filter(|&&c| c & 1 == 0 && c != 0).min().unwrap();

    return o - e;
}
