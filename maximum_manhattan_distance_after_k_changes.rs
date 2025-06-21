pub fn max_distance(s: String, k: i32) -> i32 {

    let mut counts: [i32; 4] = [0,0,0,0]; // N, S, E, W
    let mut ans = 0;

    for c in s.chars() {
        match c {
            'N' => counts[0] += 1,
            'S' => counts[1] += 1,
            'E' => counts[2] += 1,
            'W' => counts[3] += 1,
            _ => panic!("wtf"),
        }

        let v = counts[0].min(counts[1]).min(k);
        let h = counts[2].min(counts[3]).min(k - v);

        let t = (counts[0] - counts[1]).abs() + 2 * v  +  (counts[2] - counts[3]).abs() + 2 * h;
        ans = ans.max(t);

    }
    
    return ans;
}

#[test]
fn tests() {
    max_distance(String::from("NWSE"), 1);
}
