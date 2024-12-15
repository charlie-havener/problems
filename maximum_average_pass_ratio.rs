use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct MyF64(f64);

impl std::cmp::Eq for MyF64 {}

impl std::cmp::PartialOrd for MyF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::Ord for MyF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Debug for MyF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}", self.0)?;
        Ok(())
    }
}


pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {

    let get_d = | passes: i32, total: i32 | -> MyF64 {
        MyF64( (passes as f64 + 1.0)/(total as f64 + 1.0) - passes as f64 / total as f64 )
    };

    let mut bh: BinaryHeap<(MyF64, i32, i32)> = classes.iter().fold(BinaryHeap::with_capacity(classes.len()), |mut acc, class| {
        let d = get_d(class[0], class[1]);
        acc.push((d, class[0], class[1]));
        acc
    });
    println!("init: {:?}", bh);

    for _ in 0..extra_students {
        let (_, p, t) = bh.pop().unwrap();
        let new_d = get_d(p+1, t+1);
        bh.push((new_d, p+1, t+1));
        println!("{:?}", bh);
    }

    let mut total: f64 = 0.0;
    while let Some((_, p, t)) = bh.pop() {
        total += (p as f64) / (t as f64);
    }

    return total / classes.len() as f64;
}

#[test]
fn tests() {
    let classes = vec![vec![1,2],vec![3,5],vec![2,2]];
    let extra_students = 2;
    println!("{}", max_average_ratio(classes, extra_students));

    let classes = vec![vec![2,4],vec![3,9],vec![4,5],vec![2,10]];
    let extra_students = 4;
    println!("{}", max_average_ratio(classes, extra_students));
}
