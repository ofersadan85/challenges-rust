use itertools::Itertools;

pub fn is_prime(n: u128) -> bool {
    let max_div: u128 = (n as f64).sqrt() as u128 + 1;
    for div in 2..max_div {
        if n % div == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factors(n: u128) -> Vec<u128> {
    let mut n: u128 = n;
    let mut div: u128 = 2;
    let mut result: Vec<u128> = Vec::new();
    let max_div: f64 = (n as f64).sqrt();
    while n > 1 {
        if div > max_div as u128 {
            result.push(n);
            break;
        } else if n % div == 0 {
            result.push(div);
            n = n / div;
            div = 2;
        } else {
            div += 1;
        }
    }
    result
}

pub fn gcd(numbers: Vec<u128>) -> u128 {
    // returns the greatest common divisor of n numbers
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = gcd(numbers[1..].to_vec());
    gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

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

pub fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    let exp = gcd(prime_factors(n as u128)
        .iter()
        .counts()
        .values()
        .cloned()
        .collect());
    if exp > 1 {
        Some(((n as f64).powf(1.0 / exp as f64).round() as u64, exp as u32))
    } else {
        None
    }
}
