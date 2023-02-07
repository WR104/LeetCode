pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut subset: Vec<i32> = vec![];
    let mut candidates = candidates;
    candidates.sort_unstable();
    
    helper1(0, target, &candidates, &mut subset, &mut result);
    return result; 
}

fn helper1(index: usize, target: i32, candidates: &Vec<i32>, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>){
    use std::cmp::Ordering::*;
    match target.cmp(&0){
        Equal => {
            result.push(subset.to_vec());
        }
        Greater => {
            for i in index..candidates.len() {
                if i > index && candidates[i] == candidates[i - 1] {
                    continue;
                }
                subset.push(candidates[i]);
                helper1(i + 1, target - candidates[i], candidates, subset, result);
                subset.pop();
            }
        }
        Less => {}
    } 
}