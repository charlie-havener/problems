pub fn can_construct(s: String, mut k: i32) -> bool {

    if k as usize > s.len() { return false }

    let mut counts = [0; 26];
    for b in s.as_bytes() {
        counts[(b - b'a') as usize] += 1;
    }
    let odds = counts.iter().filter(|c| *c % 2 == 1).count() as i32;

    if odds > k { return false }
    k -= odds;

    let rem = s.len() as i32 - odds;
    if k > rem { return false }
    return true;
}
