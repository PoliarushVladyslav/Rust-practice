fn main() {
    let rotated = rotate("abcdefgh".to_string(), 2);
    println!("Rotated: {}", rotated); // ghabcdef
}

fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    let len = len as isize;
    let shift = ((n % len) + len) % len; // нормализуем сдвиг
    let split_at = (len - shift) as usize;
    let (first, second) = s.split_at(split_at);
    format!("{}{}", second, first)
}
