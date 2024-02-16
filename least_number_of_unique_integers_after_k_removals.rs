use std::collections::HashMap;

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let hm = arr.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v).and_modify(|count| *count += 1).or_insert(1);
        return acc;
    });

    let mut v = hm.values().into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|v1,v2| v1.cmp(v2));
    
    if k == 0 { return v.len() as i32 }
    let mut s = 0;
    for idx in 0..v.len() {
        s += v[idx];
        if s > k { return (v.len() - idx) as i32 }
    }

    return 0;
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, find_least_num_of_unique_ints(vec![5,5,4], 1));
        assert_eq!(2, find_least_num_of_unique_ints(vec![4,3,1,1,3,3,2], 3));
        assert_eq!(1, find_least_num_of_unique_ints(vec![1,2,2,3,3,3], 4));
        assert_eq!(1, find_least_num_of_unique_ints(vec![1,2,2,3,3,3], 3));
        assert_eq!(2, find_least_num_of_unique_ints(vec![1,2,2,3,3,3], 2));
        assert_eq!(3, find_least_num_of_unique_ints(vec![1,2,2,3,3,3], 0));
        assert_eq!(0, find_least_num_of_unique_ints(vec![1,2,2,3,3,3], 6));
    }
}
