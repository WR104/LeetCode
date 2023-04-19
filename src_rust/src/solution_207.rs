pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut indegrees = vec![0; num_courses as usize];
    let mut graph = vec![Vec::new(); num_courses as usize];

    for edge in &prerequisites {
        let from = edge[1] as usize;
        let to = edge[0] as usize;
        indegrees[to] += 1;
        graph[from].push(to);
    }

    let mut queue = std::collections::VecDeque::new();
    for i in 0..num_courses as usize {
        if indegrees[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut count = 0;
    while let Some(node) = queue.pop_front() {
        count += 1;
        for neighbor in &graph[node] {
            indegrees[*neighbor] -= 1;
            if indegrees[*neighbor] == 0 {
                queue.push_back(*neighbor);
            }
        }
    }

    count == num_courses as usize
}
