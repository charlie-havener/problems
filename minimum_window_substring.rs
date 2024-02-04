use std::collections::HashMap;

#[derive(Debug)]
struct SolutionVec {
    vec: Vec<(char, usize)>,
    ptr: usize,
}

impl SolutionVec {
    fn new() -> Self {
        Self { vec: Vec::new(), ptr: 0 }
    }
}

#[derive(Clone)]
struct HashMapCount {
    hm: HashMap<char, i32>,
    sum: i32,
}

impl HashMapCount {
    fn new() -> Self {
        Self { hm: HashMap::new(), sum: 0 }
    }
}

pub fn min_window(s: String, t: String) -> String {
    let mut pq = SolutionVec::new();
    let mut ans = (usize::MIN, usize::MAX);

    
    // a hashmap of the chars in t and their respective counts
    let target = t.chars().fold(HashMapCount::new(), |mut acc, c| {
        acc.hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
        acc.sum += 1;
        acc
    });
    let mut target_find = target.clone();
    let mut counts= HashMapCount::new();
    
    for (idx, c) in s.chars().enumerate() {
        if let Some(_) = target.hm.get(&c) {
            counts.hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
            counts.sum += 1;
            target_find.hm.entry(c).and_modify(|count| *count -= 1);
            if target_find.hm.get(&c).unwrap() >= &0 { target_find.sum -= 1 }
            pq.vec.push((c, idx));

            while target.sum < counts.sum {
                let head = pq.vec[pq.ptr].0;
                // safet to unwrap since we only insert valid characters.
                if target.hm.get(&head).unwrap() < counts.hm.get(&head).unwrap() {
                    pq.ptr += 1;
                    counts.hm.entry(head).and_modify(|count| *count -= 1);
                    counts.sum -= 1;
                } else { break; }
            }

            if target_find.sum == 0 {
                if ans.1 - ans.0 > pq.vec.last().unwrap().1 - pq.vec[pq.ptr].1 {
                    ans = (pq.vec[pq.ptr].1, pq.vec.last().unwrap().1)
                }
            }

            println!("{:?}, {:?}, {:?}, {:?}", pq, pq.ptr, ans, target_find.sum);
        }
    }

    if ans.1 == usize::MAX { return String::from("") }
    return s[ans.0..=ans.1].to_string();
}


#[cfg(test)]
mod test {
    use super::min_window;

    #[test]
    fn test() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        assert_eq!(String::from("BANC"), min_window(s, t));
        
        let s = String::from("ADOBECODEBANC");
        let t = String::from("AbC");
        assert_eq!(String::from(""), min_window(s, t));

        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABBC");
        assert_eq!(String::from("BECODEBA"), min_window(s, t));
        
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABBCC");
        assert_eq!(String::from("BECODEBANC"), min_window(s, t));

        let s = String::from("a");
        let t = String::from("a");
        assert_eq!(String::from("a"), min_window(s, t));

        let s = String::from("a");
        let t = String::from("aa");
        assert_eq!(String::from(""), min_window(s, t));
    }
}
