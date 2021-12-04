use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Board {
    board: [u8; 25],
    filled: [bool; 25],
    dict: HashMap<u8, usize>,
}

impl Board {
    fn new() -> Board {
        Board {
            board: [0; 25],
            filled: [false; 25],
            dict: HashMap::new(),
        }
    }

    fn mark(&mut self, num: u8) {
        if let Some(&i) = self.dict.get(&num) {
            self.filled[i] = true;
        }
    }

    fn did_win(&self) -> bool {
        for a in 0..5 {
            let mut row = true;
            let mut col = true;
            for b in 0..5 {
                let i = coords_to_index((a, b));
                let j = coords_to_index((b, a));
                row = row && self.filled[i];
                col = col && self.filled[j];
            }
            if row {
                return true;
            }
            if col {
                return true;
            }
        }
        false
    }

    fn solution(&self) -> u64 {
        let mut res = 0u64;
        self.filled
            .iter()
            .enumerate()
            .filter(|(_, &j)| !j)
            .map(|(i, _)| i)
            .for_each(|i| {
                res += self.board[i] as u64;
            });
        res
    }
}

#[derive(Debug, Clone)]
struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn new() -> Game {
        Game {
            boards: Vec::new(),
        }
    }

    fn set_game(&mut self) {
        while let Some(b) = read_board() {
            self.boards.push(b);
        }
    }

    fn play_until_win(&mut self, nums: &[u8]) -> (Option<Board>, Option<u8>) {
        let mut ret = (None, None);
        'outer:
        for num in nums {
            for b in &mut self.boards {
                b.mark(*num);
                if b.did_win() {
                    ret.0 = Some(b.clone());
                    ret.1 = Some(*num);
                    break 'outer;
                }
            }
        }
        ret
    }

    fn play_level_2(&mut self, nums: &[u8]) -> (Option<Board>, Option<u8>) {
        let mut ret = (None, None);
        for num in nums {
            let remaining = self.boards.iter().filter(|b| !b.did_win()).count();
            if remaining == 0 {
                break;
            }
            for b in &mut self.boards {
                if b.did_win() {
                    continue;
                }
                b.mark(*num);
                if b.did_win() && remaining == 1 {
                    ret = (Some(b.clone()), Some(*num));
                }
            }
        }
        ret
    }
}

fn main() {
    let nums = read_numbers();
    let mut game = Game::new();
    game.set_game();

    let (b, num) = game.clone().play_until_win(&nums);
    let b = b.unwrap();
    let num = num.unwrap();
    println!("Level 1: {}", b.solution() * num as u64);

    let (board_lvl_2, num) = game.play_level_2(&nums);
    let board_lvl_2 = board_lvl_2.unwrap();
    let num = num.unwrap();
    println!("Level 2: {}", board_lvl_2.solution() * num as u64);
}

fn read_numbers() -> Vec<u8> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .unwrap();

    input.trim().split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn read_board() -> Option<Board> {
    let mut input = String::new();
    let input_handle = std::io::stdin();
    match input_handle.read_line(&mut input) {
        Ok(0) | Err(_) => return None,
        _ => {}
    };
    let mut b = Board::new();
    let mut index = 0;
    for _ in 0..5 {
        input.clear();
        input_handle.read_line(&mut input).unwrap();
        input
            .trim()
            .split(' ')
            .filter_map(|x| x.parse::<u8>().ok() )
            .for_each(|i| {
                b.board[index] = i;
                b.dict.insert(i, index);
                index += 1
            });
    }
    Some(b)
}

fn coords_to_index(tup: (u8, u8)) -> usize {
    (tup.0 * 5 + tup.1) as usize
}
