use std::cmp::min;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    let total_weight = n * (n + 1) / 2;
    let average = total_weight / n;
    for i in 0..n {
        shipments[i] = average;
    }
    shipments
}

fn is_possible_to_balance(shipments: &Vec<u32>) -> bool {
    let total_weight: u32 = shipments.iter().sum();
    total_weight % shipments.len() as u32 == 0
}

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total_weight: u32 = shipments.iter().sum();
    let average = total_weight / shipments.len() as u32;
    
    let mut moves = 0;
    let mut balances = 0;
    for &weight in shipments.iter() {
        balances += weight - average;
        moves += balances.abs() as usize;
    }

    moves
}

fn main() {
    let shipments = vec![8, 2, 2, 4, 4];
    println!("Shipment: {:?}", shipments);
    let total_weight: u32 = shipments.iter().sum();
    let average = total_weight / shipments.len() as u32;
    println!("total = {}", total_weight);
    println!("average = {}", average);
    println!("Answer = {}", count_permutation(&shipments));

    let shipments2 = vec![9, 3, 7, 2, 9];
    println!("\nShipment: {:?}", shipments2);
    let total_weight2: u32 = shipments2.iter().sum();
    let average2 = total_weight2 / shipments2.len() as u32;
    println!("total = {}", total_weight2);
    println!("average = {}", average2);
    println!("Answer = {}", count_permutation(&shipments2));
}