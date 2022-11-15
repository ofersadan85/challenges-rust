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
