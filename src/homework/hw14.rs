use std::collections::HashSet;

fn generate_gray_codes(n: u32) -> Vec<u32> {
    (0..1 << n).map(|i| i ^ (i >> 1)).collect()
}

fn gray_code_to_point(code: u32, n: u32) -> (u32, u32) {
    let half = n / 2;
    let x = (code >> half) & ((1 << half) - 1);
    let y = code & ((1 << half) - 1);
    (x, y)
}

fn calculate_area(points: &HashSet<(u32, u32)>) -> u32 {
    if points.is_empty() {
        return 0;
    }
    let (min_x, max_x) = points.iter().fold((u32::MAX, u32::MIN), |(min_x, max_x), &(x, _)| {
        (min_x.min(x), max_x.max(x))
    });
    let (min_y, max_y) = points.iter().fold((u32::MAX, u32::MIN), |(min_y, max_y), &(_, y)| {
        (min_y.min(y), max_y.max(y))
    });
    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn main() {
    let n = 4;
    let gray_codes = generate_gray_codes(n);
    let mut points = HashSet::new();
    for code in gray_codes {
        let point = gray_code_to_point(code, n);
        points.insert(point);
    }
    let area = calculate_area(&points);
    println!("Площа, зайнята кодами Грея: {}", area);
}