struct ProductOfNumbers {
    nums: Vec<i32>
}

impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            nums: Vec::new()
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        }
        else {
            self.nums.push(self.nums.last().unwrap_or(&1) * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        if k as usize == self.nums.len() {
            return *self.nums.last().unwrap();
        }
        else if self.nums.len() > k as usize {
            return self.nums.last().unwrap() / self.nums[self.nums.len() - k as usize - 1];
        }
        else {
            return 0;
        }
    }

}
