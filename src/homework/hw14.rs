fn gray(n: u8) -> Vec<String> {
    let size = 1 << n; // 2^n
    (0..size)
        .map(|i| format!("{:0width$b}", i, width = n as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "10", "11"]),
            (3, vec!["000", "001", "010", "011", "100", "101", "110", "111"]),
            (4, vec![
                "0000", "0001", "0010", "0011",
                "0100", "0101", "0110", "0111",
                "1000", "1001", "1010", "1011",
                "1100", "1101", "1110", "1111"
            ]),
        ];

        for (n, out) in test_data {
            let result = gray(n);
            assert_eq!(result, out);
        }
    }
}

fn main() {
    println!("{:?}", gray(3)); // ["000", "001", "010", "011", "100", "101", "110", "111"]
}
