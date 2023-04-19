pub fn max_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max_prod = vec![nums[0]];
    let mut min_prod = vec![nums[0]];
    let mut res = nums[0];
    
    for i in 1..n {
        let max_val = max_prod[i-1] * nums[i];
        let min_val = min_prod[i-1] * nums[i];
        
        max_prod.push(nums[i].max(max_val).max(min_val));
        min_prod.push(nums[i].min(max_val).min(min_val));
        
        res = res.max(max_prod[i]);
    }
    
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product(vec![2,3,-2,4]), 6);
        assert_eq!(max_product(vec![-2,0,-1]), 0);
        assert_eq!(max_product(vec![-2]), -2);
        assert_eq!(max_product(vec![0,2]), 2);
    }
}
