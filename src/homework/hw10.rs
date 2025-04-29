fn is_palindromic_number(n: i32) -> bool {
    if n < 0 {
        return false; // від'ємні числа не паліндроми
    }

    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    let numbers = [121, 12321, 123, -121, 0, 10, 123454321];

    for &n in &numbers {
        println!("{} -> {}", n, is_palindromic_number(n));
    }
}
