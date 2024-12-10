use std::collections::BTreeMap;

pub fn max_area(height: i32, positions: Vec<i32>, mut directions: String) -> i64 {

    let mut map: BTreeMap<i32, Vec<usize>> = BTreeMap::new();

    // need the time until top and bottom of each, store in BTreeMap so it's ordered
    let mut sum: i64 = 0;
    let mut adj = 0;
    for (idx, val) in positions.iter().enumerate() {
        sum += *val as i64;
        match directions.as_bytes()[idx] {
            b'U' => {
                adj += 1;
                map.entry(height - *val).and_modify(|v| v.push(idx)).or_insert(vec![idx]);
                map.entry(height - *val + height).and_modify(|v| v.push(idx)).or_insert(vec![idx]);
            },
            b'D' => {
                adj -= 1;
                map.entry(*val + height).and_modify(|v| v.push(idx)).or_insert(vec![idx]);
                map.entry(*val).and_modify(|v| v.push(idx)).or_insert(vec![idx]);
            },
            _ => unreachable!(),
        }
    }

    // go through each critical time and adjust changes based on elapsed
    let mut ans: i64 = sum;
    let mut curr = 0;
    let mut it = map.iter();
    while let Some((time, idxs)) = it.next() {
        let elapsed = time - curr;
        sum += elapsed as i64 * adj as i64;
        curr = *time;
        ans = ans.max(sum);
        for i in idxs {

            // Safety: we are replacing a u8 with another u8
            unsafe {
                let d = directions.as_bytes_mut();
                match d[*i] {
                    b'U' => { adj -= 2; d[*i] = b'D'; },
                    b'D' => { adj += 2; d[*i] = b'U'; },
                    _ => unreachable!(),
                }
            }

        }
    }

    return ans;
}
