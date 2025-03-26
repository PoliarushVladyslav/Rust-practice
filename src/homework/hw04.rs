const WIDTH: usize  = 11; // Ширина конверта
const HEIGHT: usize  = 6; // Висота верхньої половини

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        output.push_str(&" ".repeat(HEIGHT - i - 1));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }
    for i in (0..HEIGHT - 1).rev() {
        output.push_str(&" ".repeat(HEIGHT - i - 1));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    print!("{}", output);
}