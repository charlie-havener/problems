use std::collections::BinaryHeap;

pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    // sort buildings by lowest left, highest height
    buildings.sort_by_key(|k| (k[0], -1 * k[2]));
    
    for idx in 0..buildings.len() {
        let (left, right, height) = (buildings[idx][0], buildings[idx][1], buildings[idx][2]);
        
        //pop off what we can
        let mut curr_right = i32::MIN;
        while pq.len() > 0 && left > pq.peek().unwrap().1 {
            let (top_height, top_right) = pq.pop().unwrap();
            if top_right > curr_right  && curr_right != i32::MIN {
                ans.push(vec![curr_right, top_height]);
                println!("{:?}", ans.last());
            }
            curr_right = curr_right.max(top_right);
        }

        if pq.is_empty() && curr_right != i32::MIN { ans.push(vec![curr_right, 0]) }

        if !pq.is_empty() {
            let (top_height, top_right) = pq.peek().unwrap();

            if height > *top_height {
                ans.push(vec![left, height]);
            }
        } else {
            ans.push(vec![left, height]);
        }
        pq.push((height, right));
    }

    //added all building now pop the rest off
    let mut curr_right = i32::MIN;
    while pq.len() > 0 {
        let (top_height, top_right) = pq.pop().unwrap();
        if top_right > curr_right  && curr_right != i32::MIN {
            ans.push(vec![curr_right, top_height]);
            println!("{:?}", ans.last());
        }
        curr_right = curr_right.max(top_right);
    }
    ans.push(vec![curr_right, 0]);


    println!("ans := {:?}", ans);
    return ans;
}

#[test]
fn test() {
    let v = vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]];
    get_skyline(v);
    let v = vec![vec![0,2,3],vec![2,5,3]];
    get_skyline(v);
}

