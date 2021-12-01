fn main() {
    let nums = read_numbers();
    println!("{}", solve_lvl_1(&nums));
    println!("{}", solve_lvl_2(&nums));
}

fn read_numbers() -> Vec<u32> {
    let mut input = String::new();
    let mut nums = Vec::<u32>::new();
    loop {
        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => break,
            _ => {}
        };
        match input.trim().parse::<u32>() {
            Ok(n) => {
                nums.push(n);
            },
            Err(_) => break,
        }
    }
    return nums
}

fn solve_lvl_1(nums: &[u32]) -> u32 {
    nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(&i, &j)| { if i < j { 1 } else { 0 } })
        .sum()
}

fn solve_lvl_2(nums: &[u32]) -> u32 {
    let v: Vec<u32> = nums
        .windows(3)
        .map(|sl| { sl.iter().sum() })
        .collect();
    solve_lvl_1(&v)
}
