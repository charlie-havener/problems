use std::collections::BinaryHeap;

pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    //println!("starting..........");
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut prev = i32::MAX;
    let mut remaining_bricks = bricks;
    for idx in 0..heights.len() {
        if remaining_bricks < 0 {
            //println!("exit @ {idx}");
            return idx as i32 - 2;
        }
        if heights[idx] >= prev {
            let diff = heights[idx] - prev;
            if (heap.len() as i32) < ladders {
                //println!("added to heap");
                heap.push(diff * -1);
            } else {
                if let Some(smallest_ladder) = heap.peek() {
                    if *smallest_ladder > (diff * -1) {
                        remaining_bricks += heap.pop().unwrap();
                        heap.push(diff * -1);
                    } else {
                        remaining_bricks -= diff;
                    }
                }else {
                    remaining_bricks -= diff;
                }
            }
        }
        //println!("{remaining_bricks}");
        prev = heights[idx];
    }

    //println!("uhhh");
    return if remaining_bricks >= 0 {heights.len() as i32 - 1}
    else {heights.len() as i32 - 2};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let heights = vec![4,2,7,6,9,14,12];
        let bricks = 5;
        let ladders = 1;
        assert_eq!(4, furthest_building(heights, bricks, ladders));

        let heights = vec![4,12,2,7,3,18,20,3,19];
        let bricks = 10;
        let ladders = 2;
        assert_eq!(7, furthest_building(heights, bricks, ladders));

        let heights = vec![14,3,19,3];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(3, furthest_building(heights, bricks, ladders));

        let heights = vec![1,93,19,3];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(0, furthest_building(heights, bricks, ladders));

        let heights = vec![1,93];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(0, furthest_building(heights, bricks, ladders));

        let heights = vec![1];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(0, furthest_building(heights, bricks, ladders));
    }
}
