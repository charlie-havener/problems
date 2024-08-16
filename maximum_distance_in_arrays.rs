pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {

    let mut lows = [(i32::MAX, 0); 2];
    let mut highs = [(i32::MIN, 0); 2];

    for (idx, a) in arrays.iter().enumerate() {
    
        //println!("\narray: {a:?}");

        let l = a[0];
        let h = *a.last().unwrap();

        //println!("(l,h): ({l},{h})");

        if l <= lows[0].0 {
            lows = [(l,idx), lows[0]];
        } else if l < lows[1].0 {
            lows = [lows[0], (l,idx)];
        }

        if h >= highs[0].0 {
            highs = [(h,idx), highs[0]];
        } else if h > highs[1].0 {
            highs = [highs[0], (h,idx)];
        }

        //println!("lows: {lows:?}, highs: {highs:?}");
    }
    //println!("{highs:?}");
    //println!("{lows:?}");

    if lows[0].1 != highs[0].1 {
        return highs[0].0 - lows[0].0;
    } else {
        return (highs[0].0 - lows[1].0).max(highs[1].0 - lows[0].0);
    }
}

#[test]
fn tests() {
    let a = vec![vec![1,2,3],vec![4,5],vec![1,2,3]];
    assert_eq!(4, max_distance(a));

    let a = vec![vec![1],vec![1]];
    assert_eq!(0, max_distance(a));
}
