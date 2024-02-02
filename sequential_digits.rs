pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let min_digits = low.checked_ilog10().unwrap_or(0) + 1;
    let max_digits = high.checked_ilog10().unwrap_or(0) + 1;

    for digits in min_digits..=max_digits {
        let mut num: i32 = 0;
        let mut ones: i32 = 0;
        for p in 0..digits {
            num += (digits - p) as i32 * 10_u32.pow(p as u32) as i32; 
            ones += 1 * 10_u32.pow(p as u32) as i32; 
        }

        for _ in 0..(10-digits) {
            if num > high { return ans; }
            if num >= low { ans.push(num); }
            num += ones;
        }
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::sequential_digits;

    #[test]
    fn test() {
        assert_eq!(vec![123,234], sequential_digits(100,300));
        assert_eq!(vec![1234,2345,3456,4567,5678,6789,12345], sequential_digits(1000,13000));
    }
}
