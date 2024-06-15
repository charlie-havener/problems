use std::collections::BinaryHeap;

pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let l = profits.len();
    let mut arr = (0..l).fold(Vec::with_capacity(l), |mut acc, idx| {
        acc.push((capital[idx], profits[idx]));
        acc
    });
    arr.sort_unstable();

    let mut pq = BinaryHeap::new();
    let mut ptr = 0;

    loop {

        while ptr < l && arr[ptr].0 <= w {
            pq.push(arr[ptr].1);
            ptr += 1;
        }

        if pq.len() == 0 {
            return w;
        }
        else {
            w += pq.pop().unwrap();
            k -= 1;
        }

        if k == 0 {
            return w;
        }
    }
}

#[test]
fn tests() {
    let k = 2;
    let w = 0;
    let p = vec![1,2,3];
    let c = vec![0,1,1];
    assert_eq!(4, find_maximized_capital(k,w,p,c));

    let k = 3;
    let w = 0;
    let p = vec![1,2,3];
    let c = vec![0,1,2];
    assert_eq!(6, find_maximized_capital(k,w,p,c));

    let k = 7;
    let w = 0;
    let p = vec![1,2,3];
    let c = vec![0,1,2];
    assert_eq!(6, find_maximized_capital(k,w,p,c));

    let k = 7;
    let w = 0;
    let p = vec![1,2,3];
    let c = vec![1,2,3];
    assert_eq!(0, find_maximized_capital(k,w,p,c));
}
