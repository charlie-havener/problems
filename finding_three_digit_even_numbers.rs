pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {

    let mut counts = [0; 10];
    for d in digits {
        counts[d as usize] += 1;
    }

    let mut ans = Vec::new();
    for a in 1..=9 {
        if counts[a] == 0 { continue; }
        counts[a] -= 1;
        for b in 0..=9 {
            if counts[b] == 0 { continue; }
            counts[b] -= 1;
            for c in 0..=4 {
                if counts[c*2] == 0 { continue }
                counts[c*2] -= 1;
                ans.push((a*100 + b*10 + c*2) as i32);
                counts[c*2] += 1;
            }
            counts[b] += 1;
        }
        counts[a] += 1;
    }
    return ans;
}
