use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = data[0] + data[1];
    let mut index = 0;
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            index = i;
        }
    }
    (index, min_sum)
}

fn print_vector(data: &[i32]) {
    println!("Вектор:");
    for (i, val) in data.iter().enumerate() {
        print!("{:>3}{}", val, if i != data.len() - 1 { ", " } else { "\n" });
    }
}

fn main() {
    let vec = gen_random_vector(20);
    print_vector(&vec);
    let (index, min_sum) = min_adjacent_sum(&vec);
    println!(
        "Мінімальна сума пари: {} + {} = {} (індекси [{}], [{}])",
        vec[index], vec[index + 1], min_sum, index, index + 1
    );
}