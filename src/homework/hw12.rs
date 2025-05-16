use rand::Rng;

/// Підрахунок мінімальної кількості переміщень вантажу для рівномірного розподілу.
/// Повертає usize::MAX, якщо розподіл неможливий.
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    if total as usize % n != 0 {
        return usize::MAX; // Нерівномірний розподіл неможливий
    }
    let average = total / n as u32;
    let mut moves = 0;
    let mut diff = 0;
    for &shipment in shipments {
        diff += shipment as i32 - average as i32;
        moves += diff.abs() as usize;
    }
    moves
}

/// Безпечна версія: повертає None, якщо розподіл неможливий.
pub fn count_permutation_safe(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    if total as usize % n != 0 {
        return None;
    }
    let average = total / n as u32;
    let mut moves = 0;
    let mut diff = 0;
    for &shipment in shipments {
        diff += shipment as i32 - average as i32;
        moves += diff.abs() as usize;
    }
    Some(moves)
}

/// Генерація вектора вантажів, що можуть бути розподілені рівномірно
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg: u32 = rng.gen_range(1..10);
    let mut shipments = vec![avg; n];

    for _ in 0..n / 2 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 0 {
            shipments[i] -= 1;
            shipments[j] += 1;
        }
    }
    shipments
}

/// Приклади
fn main() {
    println!("Running manual tests...");

    let shipments1 = vec![8, 2, 2, 4, 4];
    let result1 = count_permutation(&shipments1);
    println!("Test 1: {:?}", result1 == 4);

    let shipments2 = vec![9, 3, 7, 2, 9];
    let result2 = count_permutation(&shipments2);
    println!("Test 2: {:?}", result2 == 7);

    let shipments3 = vec![1, 2, 3];
    let result3 = count_permutation_safe(&shipments3);
    println!("Test 3: {:?}", result3.is_none());

    let data = gen_shipments(10);
    let total: u32 = data.iter().sum();
    println!(
        "Test 4: Generated shipments divisible evenly? {:?}",
        total as usize % data.len() == 0
    );
}
