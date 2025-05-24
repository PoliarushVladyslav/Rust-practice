use itertools::Itertools;

fn find_solutions() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut solutions = Vec::new();

    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // MUHA × A = SLON
        let muha = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muha * a == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }
    }
    solutions
}

fn main() {
    let solutions = find_solutions();
    for (m, u, x, a, s, l, o, n) in &solutions {
        let slon = s * 1000 + l * 100 + o * 10 + n;

        println!("\n{}{}{}{}", m, u, x, a);
        println!("  ×   {}", a);
        println!("-------");
        println!("{:>7}", slon);
    }
    println!("\nВсього рішень: {}", solutions.len());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        let solutions = find_solutions();
        assert!(solutions.len() > 0, "Рішень не знайдено!");// Solutions: 2
    }
}
