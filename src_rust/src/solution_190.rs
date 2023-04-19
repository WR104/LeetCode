pub fn reverse_bits(x: u32) -> u32 {
    let mut res = 0;
    for i in 0..32 {
        let curr = (x >> i) & 1;
        let revs = curr << (31 - i);
        res |= revs;
    }
    res
}