fn main() {
    const HEIGHT: usize = 5;

    // Верхняя половина ромба
    for i in 0..HEIGHT {
        let zirochki = 2 * i + 1;
        let progalini = HEIGHT - i - 1;
        println!("{}{}", " ".repeat(progalini), "*".repeat(zirochki));
    }

    // Нижняя половина ромба
    for i in (0..HEIGHT - 1).rev() {
        let zirochki = 2 * i + 1;
        let progalini = HEIGHT - i - 1;
        println!("{}{}", " ".repeat(progalini), "*".repeat(zirochki));
    }
}