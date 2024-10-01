pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {

    let mods = arr.into_iter().fold(vec![0; k as usize], |mut acc, v| {
        let m = v.rem_euclid(k) as usize;
        acc[m] += 1;
        acc
    });
    
    // need an even number that are already divisible by k to make pairs
    if mods[0] % 2 == 1 { return false }

    let mut iter = mods[1..].into_iter();
    loop {
        match (iter.next(), iter.next_back()) {
            (Some(f), Some(b)) => {
                if f != b { return false }
            },

            // when k is even, there is a 'middle' element that only pairs with itself
            (Some(f), None) => {
                if f%2 != 0 { return false }
            },
            (None, None) => return true,

            // next() is called before next_back().
            (None, Some(_)) => unreachable!(),
        }
    }
}

#[test]
fn tests() {
    let arr = vec![1,2,3,4,5,10,6,7,8,9];
    let k = 5;
    assert_eq!(true, can_arrange(arr, k));

    let arr = vec![1,2,3,4,5,6];
    let k = 7;
    assert_eq!(true, can_arrange(arr, k));

    let arr = vec![1,2,3,4,5,6];
    let k = 10;
    assert_eq!(false, can_arrange(arr, k));
}
