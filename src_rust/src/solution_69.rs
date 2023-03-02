fn my_sqrt(x: i32) -> i32 {
    let x = x as u64;
    let mut left = 0;
    let mut right = x;
    while left <= right {
        let mid = (left + right) / 2;
        let mid_squared = mid * mid;
        if mid_squared == x {
            return mid as i32;
        } else if mid_squared < x {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    right as i32
}