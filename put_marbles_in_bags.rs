use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {

    let k = k as usize;

    if k == 1 { return 0 }

    // lg stores the largest elements, want access to the smallest large number at the top
    // sm stores the smallest elemens, want access to the largest small nuber at the top
    let mut lg: BinaryHeap<Reverse<i64>> =  BinaryHeap::with_capacity(k - 1);
    let mut sm: BinaryHeap<i64> = BinaryHeap::with_capacity(k - 1);
    let mut lg_sum: i64 = 0;
    let mut sm_sum: i64 = 0;

    for ws in weights.windows(2) {
        let pair = ws[0] as i64 + ws[1] as i64;

        // see if it's one of the smallest
        if sm.len() >= (k-1) && *sm.peek().unwrap() > pair {
            let t = sm.pop().unwrap();
            sm_sum = sm_sum - t + pair;
            sm.push(pair);
        }
        if sm.len() < (k-1) {
            sm.push(pair);
            sm_sum += pair;
        }

        if lg.len() >= (k-1) && lg.peek().unwrap().0 < pair {
            let t = lg.pop().unwrap().0;
            lg_sum = lg_sum - t + pair;
            lg.push(Reverse(pair));
        } 
        if lg.len() < (k-1) {
            lg.push(Reverse(pair));
            lg_sum += pair;
        }
    }
    
    println!("lg: {lg:?}");
    println!("sm: {sm:?}");

    return lg_sum - sm_sum;
}

#[test]
fn tests() {
    //let weights = vec![1,3,5,1];
    //let k = 2;
    //assert_eq!(4, put_marbles(weights, k));
    //
    //let weights = vec![1,3];
    //let k = 2;
    //assert_eq!(0, put_marbles(weights, k));

    let weights = vec![1,4,2,5,2];
    let k = 3;
    assert_eq!(3, put_marbles(weights, k));
}
