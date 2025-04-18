fn main() {
    let triangle_count = 5;

    let max_triangle_height = triangle_count + 1;

    let max_width = 2 * (max_triangle_height - 1) + 1;

    for level in 0..triangle_count {
        let height = level + 2;

        for i in 0..height {
            let stars = 2 * i + 1;
            let spaces = (max_width - stars) / 2;

            let line: String = std::iter::repeat(' ')
                .take(spaces)
                .chain(std::iter::repeat('★').take(stars))
                .collect();

            println!("{}", line);
        }
    }
}
