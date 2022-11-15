pub fn factorial(n: usize) -> u128 {
    (1..=n).reduce(|a, b| a * b).unwrap_or(1) as u128
}

pub fn pascal_row(n: usize) -> Vec<usize> {
    (0..=n)
        .map(|i| (factorial(n) / (factorial(i) * factorial(n - i))) as usize)
        .collect()
}

pub fn pascals_triangle(n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..n {
        result.append(&mut pascal_row(i))
    }
    result
}

pub fn pascals_triangle_2(n: usize) -> Vec<usize> {
    let mut rows: Vec<Vec<usize>> = vec![vec![1]];
    while rows.len() < n {
        let previous = rows.last().unwrap();
        let mut next = vec![1];
        for i in 1..previous.len() {
            next.push(previous[i - 1] + previous[i]);
        }
        next.push(1);
        rows.push(next);
    }
    rows.into_iter().flatten().collect::<Vec<usize>>()
}