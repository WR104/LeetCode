pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let (mut l, mut r, mut s) = (left, right, 0);
    while l < r { l >>= 1; r >>= 1; s += 1; }
    r << s
}
