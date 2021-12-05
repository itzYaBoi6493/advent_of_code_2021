use std::cmp;
use std::collections::HashMap;

fn main() {
    let data = read_data();
    println!("Level 1: {}", solve(&data, false));
    println!("Level 2: {}", solve(&data, true));
} 

fn solve(data: &[String], lvl_2: bool) -> usize {
    let mut h = HashMap::<(u16, u16), u64>::new();
    data
        .iter()
        .map(|s| parse_str(s, lvl_2))
        .flatten()
        .for_each(|tup| {
            let i = h.entry(tup).or_insert(0);
            *i += 1;
        });
    h.iter().filter(|(_, &c)| c >= 2).count()
}

fn read_data() -> Vec<String> {
    let mut input = String::new();
    let mut results = Vec::<String>::new();
    loop {
        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => break,
            _ => {}
        }
        results.push(input.trim().to_owned());
    }
    results
}

fn parse_str(s: &str, lvl_2: bool) -> Vec<(u16, u16)> {
    let mut it = s.split(' ');
    let a = it.next().unwrap().split(',').filter_map(|i| i.parse::<u16>().ok()).collect::<Vec<u16>>();
    let b = it.last().unwrap().split(',').filter_map(|i| i.parse::<u16>().ok()).collect::<Vec<u16>>();

    let mut results = Vec::<(u16, u16)>::new();
    if a[0] == b[0] {
        let start = cmp::min(a[1], b[1]);
        let end = cmp::max(a[1], b[1]);
        for i in start..=end {
            results.push((a[0], i));
        }
    } else if a[1] == b[1] {
        let start = cmp::min(a[0], b[0]);
        let end = cmp::max(a[0], b[0]);
        for i in start..=end {
            results.push((i, a[1]));
        }
    } else if lvl_2 {
        let (mut i, mut j) = (a[0], a[1]);
        while i != b[0] && j != b[1] {
            results.push((i, j));
            if a[0] > b[0] { i -= 1; } else { i += 1; };
            if a[1] > b[1] { j -= 1; } else { j += 1; };
        }
        results.push((i, j));
    }
    results
}
