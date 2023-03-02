fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    let mut components = path.split('/').collect::<Vec<_>>();
    for component in components {
        match component {
            "." | "" => {}
            ".." => {
                stack.pop();
            }
            _ => {
                stack.push(component);
            }
        }
    }
    let mut result = String::new();
    for component in stack {
        result.push('/');
        result.push_str(component);
    }
    if result.is_empty() {
        result.push('/');
    }
    result
}