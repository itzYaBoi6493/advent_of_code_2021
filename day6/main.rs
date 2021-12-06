use std::collections::HashMap;

fn main() {
    let v = read_data();
    println!("Level 1: {}", solve(&v, 80));
    println!("Level 2: {}", solve(&v, 256));
}

fn read_data() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().split(',').map(|i| i.parse::<i32>().unwrap()).collect()
}

fn solve(v: &[i32], n: usize) -> usize {
    let mut day = 0;
    let mut h = HashMap::<i32, usize>::new();
    v.iter().for_each(|i| {
        *h.entry(*i).or_insert(0) += 1;
    });

    while day < n {
        let val = *h.get(&0).unwrap_or(&0);
        for i in 0..=8 {
            *h.entry(i).or_insert(0) = *h.entry(i + 1).or_insert(0);
        }
        *h.entry(6).or_insert(0) += val;
        *h.entry(8).or_insert(0) += val;
        day += 1;
    }
    h.values().sum()
}
