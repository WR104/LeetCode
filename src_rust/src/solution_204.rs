pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let mut is_prime = vec![true; n as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..(n as f64).sqrt() as i32 + 1 {
        if is_prime[i as usize] {
            let mut j = i*i;
            while j < n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    let mut count = 0;
    for i in 2..n {
        if is_prime[i as usize] {
            count += 1;
        }
    }
    count
}