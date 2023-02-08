pub fn my_pow(x: f64, n: i32) -> f64 {
    if x == 1.0 || n == 0 {
        return 1.0;
    }

    let mut val = my_pow(x, n/2);
    val = val * val;
    if n % 2 == 0 {
        return val;
    } else {
        if n < 0 {
            return val * (1.0/x);
        }
        return val * x;
    }
}