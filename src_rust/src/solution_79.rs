pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(board: &Vec<Vec<char>>, word: &Vec<char>, i: usize, j: usize, k: usize) -> bool {
        if board[i][j] != word[k] || board[i][j] == '#'{
            return false;
        }
        if k == word.len() - 1 {
            return true;
        }
        let mut new_board = board.clone();
        new_board[i][j] = '#';
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (x, y) in directions {
            let (i, j) = (i as i32 + x, j as i32 + y);
            if i >= 0 && i < board.len() as i32 && j >= 0 && j < board[0].len() as i32
                && dfs(&new_board, word, i as usize, j as usize, k + 1) {
                return true;
            }
        }
        false
    }
    let word: Vec<char> = word.chars().collect();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == word[0] && dfs(&board, &word, i, j, 0) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("ABCCED")), true);
        assert_eq!(exist(vec![vec!['a']], String::from("a")), true);
    }
}