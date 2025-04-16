fn get_char_at(x: usize, y: usize, width: usize, height: usize) -> char {
    if y == 0 || y == height - 1 || x == 0 || x == width - 1 || x == y || x == width - 1 - y {
    if y == 0 || y == height - 1 || x == 0 || x == width - 1 || x == y * (width - 1) / (height - 1) || x == (height - 1 - y) * (width - 1) / (height - 1) {
        '*'
    } else {
        ' '
    }
}

fn main() {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const WIDTH: usize = 28;
    const HEIGHT: usize = 10;

    let mut output = String::new();

    for y in 0..HEIGHT {
        let row: String = (0..WIDTH)
            .map(|x| get_char_at(x, y, WIDTH, HEIGHT))
            .collect();
        output.push_str(&row);
        output.push('\n');
    }

    print!("{}", output);
}