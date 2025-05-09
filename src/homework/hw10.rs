fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}
fn main() {
    println!("{}", is_palindrome(121));   // true
    println!("{}", is_palindrome(123));   // false
    println!("{}", is_palindrome(1221));  // true
}
