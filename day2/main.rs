fn main() {
    let mut input = String::new();
    let input_handle = std::io::stdin();

    let (mut x, mut y) = (0u64, 0u64);
    let (mut x2, mut y2) = (0u64, 0u64);
    let mut aim = 0u64;
    loop {
        input.clear();
        match input_handle.read_line(&mut input) {
            Ok(0) | Err(_) => break,
            _ => {}
        }

        let mut it = input.trim().split(' ');
        let cmd = it.next().unwrap();
        let num = it.next().unwrap().parse::<u64>().unwrap();
        match cmd {
            "forward" => {
                // lvl1
                x += num;
                // lvl2
                x2 += num;
                y2 += aim * num;
            },
            "up" => {
                // lvl1
                y -= num;
                // lvl2
                aim -= num;
            },
            "down" => {
                // lvl1
                y += num;

                // lvl2
                aim += num;
            },
            _ => {}
        }
    }
    println!("Level 1");
    println!("x: {}, y: {}", x, y);
    println!("{}", x * y);

    println!("\n\nLevel 2");
    println!("x: {}, y: {}", x2, y2);
    println!("{}", x2 * y2);
}

