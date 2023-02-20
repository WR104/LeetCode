pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut stack: Vec<Vec<i32>> = Vec::new();
    for interval in intervals {
        if let Some(top) = stack.last_mut() {
            if interval[0] <= top[1] {
                top[1] = std::cmp::max(top[1], interval[1]);
                continue;
            }
        }
        stack.push(interval);
    }
    stack
}