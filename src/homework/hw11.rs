use rand::Rng;

/// Генерує випадковий вектор довжиною `n` з елементами в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

/// Знаходить індекс пари з мінімальною сумою в сусідніх елементах
fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    (min_index, min_sum)
}

/// Форматований вивід у консоль згідно з прикладом
fn print_result(data: &[i32], min_index: usize, min_sum: i32) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>2}. ", i);
    }
    println!();

    // Вивід значень
    print!("data:    [");
    for (i, value) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", value);
    }
    println!("]");

    // Вивід стрілки з мінімальною парою
    print!("indexes: ");
    let mut spaces = 0;
    for i in 0..min_index {
        spaces += format!("{:>2}. ", i).len();
    }
    print!("{:>width$}\\__ __/", "", width = spaces);
    println!();

    // Вивід підсумку
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    let (min_index, min_sum) = min_adjacent_sum(&data);
    print_result(&data, min_index, min_sum);
}
