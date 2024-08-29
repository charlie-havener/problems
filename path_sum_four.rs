use std::collections::HashMap;

pub fn path_sum(nums: Vec<i32>) -> i32 {
    
    let mut vals: HashMap<(i32,i32), i32> = HashMap::with_capacity(nums.len());
    for mut n in nums {
        let v = n.rem_euclid(10);
        n /= 10;
        let p = n.rem_euclid(10);
        n /= 10;
        let l = n.rem_euclid(10);

        vals.insert((l,p),v);
    }

    let mut ans = 0;

    let mut queue = vec![(1,1)];
    while let Some((l,p)) = queue.pop() {
        let v = vals.get(&(l,p)).unwrap().clone();
        let mut child_count = 0;

        if let Some(k) = vals.get(&(l+1, p*2)) {
            child_count += 1;
            queue.push((l+1, p*2));
            vals.insert((l+1, p*2), *k + v);
        }

        if let Some(k) = vals.get(&(l+1, p*2 - 1)) {
            child_count += 1;
            queue.push((l+1, p*2 - 1));
            vals.insert((l+1, p*2 - 1), *k + v);
        }

        if child_count == 0 { ans += v }
    }

    return ans;
}

#[test]
fn tests() {
    let n = vec![113,215,221];
    assert_eq!(12, path_sum(n));

    let n = vec![113,221];
    assert_eq!(4, path_sum(n));

    let n = vec![119, 219, 229, 319, 329, 339, 349, 419, 429, 439, 449, 459, 469, 479, 489];
    assert_eq!(288, path_sum(n));

    let n = vec![119, 219, 229, 319, 329, 339, 349, 419, 429, 439, 459, 469, 479, 489];
    assert_eq!(252, path_sum(n));

    let n = vec![119, 219, 229, 319, 329, 349, 419, 429, 439, 449, 459, 469, 479, 489];
    assert_eq!(216, path_sum(n));
}
