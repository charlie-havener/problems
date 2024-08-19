pub fn min_steps(n: i32) -> i32 {

    let n = n as usize;

    let mut counts = vec![i32::MAX; 4.max(n+1)];
    counts[0] = 0;
    counts[1] = 0;
    counts[2] = 2;
    counts[3] = 3;

    if n <= 3 {
        return counts[n];
    }

    for start in 2..=n {
        if counts[start] == i32::MAX {
            counts[start] = start as i32;
        }

        let mut i = start;
        let mut v = counts[i] + 1;
        loop {
            i += start;
            v += 1;
            if i > n { break }
            counts[i] = counts[i].min(v);
            //println!("  -- {i} {}", counts[i]);
        }
        
        //println!("{start} => counts: {counts:?}");
    }
    return counts[n] as i32;
}

#[test]
fn tests() {
    assert_eq!(3, min_steps(3));
    assert_eq!(997, min_steps(997));
    assert_eq!(6, min_steps(8));
    assert_eq!(17, min_steps(17));
    assert_eq!(19, min_steps(34));
    assert_eq!(8, min_steps(16));
    assert_eq!(21, min_steps(1000));
    assert_eq!(46, min_steps(999));
}
