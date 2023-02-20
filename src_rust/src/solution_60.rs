pub fn get_permutation(n: i32, k: i32) -> String {
    let mut nums = (1..=n).collect::<Vec<_>>();
    let mut res = String::new();
    let mut k = k - 1;
    
    for i in (1..=n).rev() {
        let fact = (1..i).fold(1, |acc, x| acc * x);
        let j = k / fact;
        k -= j * fact;
        res.push_str(&nums.remove(j as usize).to_string());
    }
    
    res 
}