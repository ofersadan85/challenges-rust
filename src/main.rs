fn is_prime(n: i32) -> bool {
    let max_div: i32 = (n as f32).powf(0.5) as i32 + 1;
    for div in 2..max_div {
        if n % div == 0 {
            return false;
        }
    }
    true
}

fn prime_factors(n: i32) -> Vec<i32> {
    let mut n: i32 = n;
    let mut div: i32 = 2;
    let mut result = Vec::new();
    let max_div: f32 = (n as f32).powf(0.5);
    while n > 1 {
        if div > max_div as i32 {
            result.push(n);
            break;
        } else if n % div == 0 {
            result.push(div);
            n = n / div;
            div = 2;
        } else {
            div = div + 1;
        }
    }
    result
}

fn main() {
    for n in 1..103 {
        let test = prime_factors(n);
        println!("testing {}: {:?}", n, test)
    }
    println!("testing {}: {:?}", i32::MAX - 1, prime_factors(i32::MAX - 1));
    println!("testing {}: {:?}", i32::MAX, prime_factors(i32::MAX));
    for n in 1..103 {
        let test = is_prime(n);
        println!("testing {}: {:?}", n, test)
    }
    println!("testing {}: {:?}", i32::MAX - 1, is_prime(i32::MAX - 1));
    println!("testing {}: {:?}", i32::MAX, is_prime(i32::MAX));
}
