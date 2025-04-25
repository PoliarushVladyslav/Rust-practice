fn draw_triangle(base_width: usize, indent: usize) {
    for i in 0..(base_width + 1) / 2 {
        let stars = 2 * i + 1;
        let spaces = indent + (base_width - stars) / 2;
        println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
    }
}

fn draw_tree(levels: usize) {
    let mut base_width = 1;
    for level in 0..levels {
        // Створюємо трикутники зростаючої ширини
        for row in 0..(level + 1) {
            base_width = 2 * row + 1;
            let indent = levels - row - 1;
            println!(
                "{}{}",
                " ".repeat(indent),
                "*".repeat(base_width)
            );
        }
    }

    // Малюємо стовбур
    let trunk_indent = levels - 1;
    println!("{}*", " ".repeat(trunk_indent));
}

fn main() {
    let levels = 5; // Кількість трикутників
    draw_tree(levels);
}
