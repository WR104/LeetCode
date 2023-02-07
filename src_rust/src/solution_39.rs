use std::collections::HashSet;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut curr = vec![];
    helper1(&candidates, target, &mut curr, &mut result);

    for r in result.iter_mut() {
        r.sort();
    }

    let mut set = HashSet::new();
    result
    .retain(|r| {
        let r = r.to_vec();
        if set.contains(&r) {
            false
        } else {
            set.insert(r);
            true
        }
    });
    result
}   

fn helper1(subset: &Vec<i32>, target: i32, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    for item in subset.iter() {
        if target - item == 0 {
            curr.push(*item);
            result.push(curr.to_vec());
            curr.pop();
        } else if target - item > 0 {
            curr.push(*item);
            helper1(subset, target - item, curr, result);
            curr.pop();
        }
    }
}