impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        nums.sort_unstable_by(|a, b| (b.to_owned() + a).cmp(&(a.to_owned() + b)));
        let res = nums.join("");
        if res.chars().next() == Some('0') {
            return "0".to_string();
        }
        res
    }
}

