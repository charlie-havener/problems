#[derive(Debug)]
struct NumArray {
    table: Vec<i32>,
    size: usize,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        // precompute the prefix sums
        for idx in 1..nums.len() {
            nums[idx] += nums[idx - 1]; 
        }

        let mut bit = Self {
            table: vec![0; nums.len() + 1],
            size: nums.len() + 1,
        };

        // each pos is a sum of digits
        // the number of digits summed is equal to it's rightmost set bit
        for idx in 1..bit.table.len() {
            let lsb = (idx as i32 & -(idx as i32)) as usize;
            if lsb == idx {
                bit.table[idx] = nums[idx-1];
            } else {
                bit.table[idx] = nums[idx-1] - nums[idx-1-lsb];
            }
        }

        return bit;
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut idx = index + 1;
        let delta = val - self.sum_range(index, index);
        println!("{delta}");
        while idx < self.size as i32 {
            self.table[idx as usize] += delta;
            idx += idx & -idx;
        }
    }

    fn sum(&self, mut right: i32) -> i32 {
        // go *down* the tree while idx > 0
        let mut sum = 0;
        while right > 0 {
            sum += self.table[right as usize];
            right -= right & -right;
        }
        return sum;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.sum(right+1) - self.sum(left);
    }
}

#[test]
fn tests() {
    let nums = vec![1,3,5];
    let mut n = NumArray::new(nums);
    println!("{n:?}");
    assert_eq!(9, n.sum_range(0,2));
    n.update(1,2);
    println!("{n:?}");
    assert_eq!(8, n.sum_range(0,2));

    let nums = vec![-1];
    let mut n = NumArray::new(nums);
    println!("{n:?}");
    assert_eq!(-1, n.sum_range(0,0));
    n.update(0,1);
    println!("{n:?}");
    assert_eq!(1, n.sum_range(0,0));
}
