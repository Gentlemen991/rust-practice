fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let shift = ((n % len) + len) % len;
    let split = (len - shift) as usize;

    let (left, right) = s.split_at(split);
    format!("{}{}", right, left)
}

// Для тесту
fn rotate2(s: &str, n: &isize) -> String {
    rotate(s.to_string(), *n)
}

fn main() {
    let s = "abcdefgh".to_string();
    let examples = [0, 1, -1, 2, -2, 8, -8, 10, -10];

    for &shift in &examples {
        let result = rotate(s.clone(), shift);
        println!("rotate({}, {}) = {}", s, shift, result);
    }
}
