use std::collections::{hash_map::Entry, HashMap};

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2_map: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let hm = nums2.iter().fold(HashMap::with_capacity(nums2.len()), |mut acc, &v| {
            acc.entry(v).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

        return Self {
            nums1,
            nums2,
            nums2_map: hm,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        
        let cv =  self.nums2[index as usize];

        match self.nums2_map.entry(cv) {
            Entry::Occupied(mut o) => {
                *o.get_mut() -= 1;
                if *o.get() == 0 {
                    let _ = o.remove_entry();
                }
            },
            Entry::Vacant(_) => panic!("wtf?"),
        }

        self.nums2_map.entry(cv + val).and_modify(|c| *c += 1).or_insert(1);
        self.nums2[index as usize] = cv + val;

    }

    fn count(&self, tot: i32) -> i32 {

        let mut ans = 0;
        for v in &self.nums1 {
            let target = tot - *v;
            ans += self.nums2_map.get(&target).unwrap_or(&0);
        }
        return ans;
    }
}
