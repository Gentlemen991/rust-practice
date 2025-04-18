use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Введите первое число:");
    io::stdin().read_line(&mut input1).expect("Ошибка чтения первого числа");

    println!("Введите второе число:");
    io::stdin().read_line(&mut input2).expect("Ошибка чтения второго числа");

    let num1: u64 = input1.trim().parse().expect("Неверный ввод первого числа");
    let num2: u64 = input2.trim().parse().expect("Неверный ввод второго числа");

    let result = gcd(num1, num2);
    println!("НОД (GCD) чисел {} и {} = {}", num1, num2, result);
}
