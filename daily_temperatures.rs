
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; temperatures.len()];
    
    let mut hottest = 0;
    for (idx, &val) in temperatures.iter().enumerate().rev() {
        if val >= hottest {
            hottest = val;
            continue
        }
        let mut days = 1;
        while temperatures[idx + days] <= val {
            days += ans[idx + days] as usize;
        }
        ans[idx] = days as i32;
    }
    return ans;
}


#[cfg(test)]
mod test {
    use super::daily_temperatures;

    #[test]
    fn test() {
        let temperatures =vec![73,74,75,71,69,72,76,73];
        assert_eq!(vec![1,1,4,2,1,1,0,0], daily_temperatures(temperatures));
        let temperatures =vec![30,40,50,60];
        assert_eq!(vec![1,1,1,0], daily_temperatures(temperatures));
        let temperatures =vec![30,60,90];
        assert_eq!(vec![1,1,0], daily_temperatures(temperatures));
    }
}
