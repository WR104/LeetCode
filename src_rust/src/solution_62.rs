pub fn unique_paths(m: i32, n: i32) -> i32 {
	let m = m as usize;
	let n = n as usize;
	let mut table = vec![vec![0;n]; m];

	table[0][0] = 1;
	for i in 1..m {
		table[i][0] = 1;
	}
	for j in 1..n {
		table[0][j] = 1;
	}
	for i in 1..m {
		for j in 1..n {
			table[i][j] = table[i-1][j] + table[i][j-1];
		}
	}

	table[m-1][n-1]
}
