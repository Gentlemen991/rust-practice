fn rotate(s: &str, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }

    // Зводимо зсув до діапазону [0, len)
    let shift = ((n % len) + len) % len;
    let shift = shift as usize;

    format!("{}{}", &s[len as usize - shift..], &s[..len as usize - shift])
}

// Для сумісності з вашим тестом, створимо обгортку, яка приймає String
fn rotate2(s: &str, n: &isize) -> String {
    rotate(s, *n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)|
                assert_eq!(
                    rotate2(s, n),
                    exp.to_string()
                )
            );
    }
}