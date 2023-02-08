mod solution_32;
fn main() {
    let str = String::from(")()())");
    let res = solution_32::longest_valid_parentheses(str);
    println!("{:?}", res);
}
