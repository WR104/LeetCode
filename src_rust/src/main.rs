mod solution_63;
fn main() {
    let g = vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]];
    let res = solution_63::unique_paths_with_obstacles(g);
    println!("{:?}", res);
}
