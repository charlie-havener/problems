use std::collections::BinaryHeap;

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {

    let counts: [i32; 26] = s.bytes().fold([0;26], |mut acc, b| {
        acc[(b - b'a') as usize] += 1;
        acc
    });
    let mut bh: BinaryHeap<(u8, i32)> = counts.iter().enumerate().fold(BinaryHeap::new(), |mut acc, (idx, cnt)| {
        if *cnt > 0 {
            acc.push((idx as u8, *cnt));
        }
        acc
    });

    let mut ans = String::with_capacity(s.len());
    while let Some((c, cnt)) = bh.pop() {

        // add the greatest character as many times as possible
        let to_add = repeat_limit.min(cnt);
        for _ in 0..to_add {
            ans.push((c + b'a') as char);
        }

        // if exhausted then go next loop
        if cnt <= repeat_limit { continue }

        // otherwise get the next best character and add it once
        // then add back to heap if count is still > 0
        if let Some((c2, cnt2)) = bh.pop() {
            ans.push((c2 + b'a') as char);
            if cnt2 > 1 {
                bh.push((c2, cnt2 - 1));
            }
        }
        // if there is no next character then we are done
        else {
            break;
        }

        // readd greatest charactr to heap with updated count
        bh.push((c, cnt - to_add));
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("cczazcc");
    let repeat_limit = 3;
    assert_eq!(String::from("zzcccac"), repeat_limited_string(s, repeat_limit));

    let s = String::from("aababab");
    let repeat_limit = 2;
    assert_eq!(String::from("bbabaa"), repeat_limited_string(s, repeat_limit));

    let s = String::from("zzzzzzzzaaaaa");
    let repeat_limit = 3;
    assert_eq!(String::from("zzzazzzazzaaa"), repeat_limited_string(s, repeat_limit));

    let s = String::from("zzzzzzzzaaaaa");
    let repeat_limit = 2;
    assert_eq!(String::from("zzazzazzazzaa"), repeat_limited_string(s, repeat_limit));
}
