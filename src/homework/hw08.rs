fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 10, 17, 100, 101];
    for &n in &numbers {
        println!("Число {} є простим: {}", n, is_prime(n));
    }
}
