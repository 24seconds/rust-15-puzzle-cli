use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{error::Error, time::Instant};

#[derive(PartialEq)]
pub enum GameState {
    INIT,
    PLAYING,
    PAUSED,
    DONE,
}

pub struct GameData {
    pub game_state: GameState,
    pub move_count: i32,
    pub base_time: u64,
    pub arr_state: [u16; 16],
    pub start_time: Instant,
}

impl GameData {
    pub fn new(rng: &mut ThreadRng) -> Self {
        GameData {
            game_state: GameState::INIT,
            move_count: 0,
            base_time: 0,
            arr_state: shuffle_arr(rng).unwrap(),
            start_time: Instant::now(),
        }
    }
}

pub fn handle_move_operation(game_data: &mut GameData, next_arr_state: [u16; 16], key: char) {
    if !is_state_same(game_data.arr_state, next_arr_state)
        && game_data.game_state != GameState::DONE
    {
        game_data.move_count += 1;
        game_data.arr_state = next_arr_state;
    }

    let next_game_state = handle_game_state(&game_data, key);

    game_data.base_time = update_elapsed_time(&game_data, &next_game_state);
    game_data.game_state = next_game_state;
}

pub fn handle_game_state(game_data: &GameData, char: char) -> GameState {
    let curren_state = &game_data.game_state;
    let arr_state = &game_data.arr_state;

    match curren_state {
        GameState::INIT => {
            if ['w', 'a', 's', 'd'].contains(&char) {
                GameState::PLAYING
            } else {
                GameState::INIT
            }
        }
        GameState::PLAYING => {
            let is_done = is_done(arr_state);

            if char == 'p' {
                GameState::PAUSED
            } else if is_done {
                GameState::DONE
            } else {
                GameState::PLAYING
            }
        }
        GameState::PAUSED => GameState::PLAYING,
        GameState::DONE => {
            if char == 'r' {
                GameState::INIT
            } else {
                GameState::DONE
            }
        }
    }
}

pub fn update_elapsed_time(game_data: &GameData, next_game_state: &GameState) -> u64 {
    let game_state = &game_data.game_state;
    let base_time = game_data.base_time;
    let start_time = &game_data.start_time;

    let mut updated_base_time = base_time;

    if game_state == &GameState::PLAYING
        && (next_game_state == &GameState::PAUSED || next_game_state == &GameState::DONE)
    {
        updated_base_time = base_time + start_time.elapsed().as_secs();
    }

    updated_base_time
}

fn is_state_same(arr1: [u16; 16], arr2: [u16; 16]) -> bool {
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            return false;
        }
    }

    true
}

fn shuffle_arr(rng: &mut ThreadRng) -> Result<[u16; 16], Box<dyn Error>> {
    let mut arr = [0; 16];

    (0..16).into_iter().enumerate().for_each(|args| {
        let (index, number) = args;

        arr[index] = number;
    });

    loop {
        arr.shuffle(rng);

        if is_solvable(&arr)? {
            break;
        }
    }

    Ok(arr)
}

fn is_solvable(arr: &[u16; 16]) -> Result<bool, Box<dyn Error>> {
    // solvable : blank even row (count from bottom, count start from 1) and odd count inversions
    // solvable : blank odd row (count from bottom, count start from 1) and even count inversions

    let blank_index = arr
        .iter()
        .position(|x| *x == 0)
        .ok_or("There is no blank!")?;
    let blank_row = 4 - blank_index / 4;
    let inversion_count = count_inversion(&arr);

    let solvable = if blank_row % 2 == 0 {
        // blank row is even

        inversion_count % 2 == 1
    } else {
        // blank row is odd

        inversion_count % 2 == 0
    };

    Ok(solvable)
}

fn count_inversion(arr: &[u16; 16]) -> u16 {
    let mut count = 0;

    let length = arr.len();

    for i in 0..length {
        for j in 0..length {
            if i == j {
                continue;
            }

            if arr[i] == 0 || arr[j] == 0 {
                continue;
            }

            if arr[i] > arr[j] && i < j {
                count += 1;
            }
        }
    }

    count
}

pub enum Operation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn move_tile(arr: &[u16; 16], operation: Operation) -> Result<[u16; 16], Box<dyn Error>> {
    let mut next_arr = [0; 16];

    arr.iter().enumerate().for_each(|args| {
        let (index, number) = args;
        next_arr[index] = *number;
    });

    let index_blank = arr
        .iter()
        .position(|x| *x == 0)
        .ok_or("There is no blank!")?;
    let arr_length = arr.len();

    match operation {
        Operation::UP => {
            let index_to_swap = index_blank + 4;

            if index_to_swap < arr_length {
                let temp = next_arr[index_blank];
                next_arr[index_blank] = next_arr[index_to_swap];
                next_arr[index_to_swap] = temp;
            }
        }
        Operation::DOWN => {
            let index_to_swap = index_blank as i32 - 4;

            if index_to_swap >= 0 {
                let index_to_swap = index_to_swap as usize;
                let temp = next_arr[index_blank];

                next_arr[index_blank] = next_arr[index_to_swap];
                next_arr[index_to_swap] = temp;
            }
        }
        Operation::LEFT => {
            let index_to_swap = index_blank + 1;

            if index_to_swap < arr_length && index_blank % 4 != 3 {
                let temp = next_arr[index_blank];
                next_arr[index_blank] = next_arr[index_to_swap];
                next_arr[index_to_swap] = temp;
            }
        }
        Operation::RIGHT => {
            let index_to_swap = index_blank as i32 - 1;

            if index_to_swap >= 0 && index_blank % 4 != 0 {
                let index_to_swap = index_to_swap as usize;
                let temp = next_arr[index_blank];

                next_arr[index_blank] = next_arr[index_to_swap];
                next_arr[index_to_swap] = temp;
            }
        }
    };

    Ok(next_arr)
}

fn is_done(arr_state: &[u16; 16]) -> bool {
    let result = (0..16).into_iter().all(|x| {
        if x == 15 {
            arr_state[x as usize] == 0
        } else {
            x + 1 == arr_state[x as usize]
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn count_inversion_should_correct() {
        {
            let arr = [2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
            let count = count_inversion(&arr);
            assert_eq!(count, 1);
        }

        {
            let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            let count = count_inversion(&arr);
            assert_eq!(count, 0);
        }

        {
            let arr = [13, 2, 10, 3, 1, 12, 8, 4, 5, 0, 9, 6, 15, 14, 11, 7];
            let count = count_inversion(&arr);
            assert_eq!(count, 41);
        }

        {
            let arr = [6, 13, 7, 10, 8, 9, 11, 0, 15, 2, 12, 5, 14, 3, 1, 4];
            let count = count_inversion(&arr);
            assert_eq!(count, 62);
        }

        {
            let arr = [3, 9, 1, 15, 14, 11, 4, 6, 13, 0, 10, 12, 2, 7, 8, 5];
            let count = count_inversion(&arr);
            assert_eq!(count, 56);
        }
    }

    #[test]
    fn is_solvable_should_correct() -> Result<(), Box<dyn Error>> {
        {
            let arr = [13, 2, 10, 3, 1, 12, 8, 4, 5, 0, 9, 6, 15, 14, 11, 7];
            let is_solvable = is_solvable(&arr)?;
            assert_eq!(is_solvable, true);
        }

        {
            let arr = [6, 13, 7, 10, 8, 9, 11, 0, 15, 2, 12, 5, 14, 3, 1, 4];
            let is_solvable = is_solvable(&arr)?;
            assert_eq!(is_solvable, true);
        }

        {
            let arr = [3, 9, 1, 15, 14, 11, 4, 6, 13, 0, 10, 12, 2, 7, 8, 5];
            let is_solvable = is_solvable(&arr)?;
            assert_eq!(is_solvable, false);
        }

        {
            let test_set = [
                [13, 10, 11, 6, 5, 3, 1, 4, 8, 0, 12, 2, 14, 7, 9, 15],
                [9, 2, 15, 13, 7, 4, 12, 6, 8, 1, 0, 14, 5, 10, 3, 11],
                [4, 8, 7, 12, 5, 0, 13, 15, 9, 1, 6, 3, 11, 14, 10, 2],
                [7, 10, 11, 1, 0, 9, 3, 4, 5, 8, 13, 2, 14, 6, 12, 15],
                [14, 11, 8, 15, 12, 5, 13, 3, 6, 2, 9, 0, 1, 7, 10, 4],
                [4, 12, 15, 9, 2, 13, 14, 3, 5, 7, 8, 6, 11, 1, 10, 0],
                [12, 14, 2, 11, 1, 7, 0, 10, 6, 5, 13, 4, 8, 9, 15, 3],
                [4, 13, 8, 7, 10, 6, 2, 9, 5, 0, 14, 11, 12, 15, 1, 3],
                [4, 5, 11, 13, 3, 7, 8, 12, 0, 14, 2, 6, 10, 15, 1, 9],
                [15, 4, 3, 6, 2, 7, 5, 1, 8, 11, 0, 14, 13, 9, 10, 12],
                [7, 10, 13, 5, 6, 8, 11, 0, 1, 2, 12, 14, 3, 4, 15, 9],
                [10, 9, 0, 11, 1, 6, 15, 7, 4, 5, 2, 12, 14, 13, 3, 8],
                [4, 15, 14, 8, 10, 9, 3, 12, 7, 6, 13, 0, 2, 11, 1, 5],
                [9, 7, 15, 12, 8, 6, 13, 5, 14, 2, 11, 1, 4, 0, 10, 3],
                [3, 5, 14, 4, 0, 10, 12, 7, 15, 9, 6, 11, 2, 1, 13, 8],
                [5, 15, 3, 10, 9, 8, 7, 14, 4, 13, 12, 2, 0, 1, 6, 11],
                [5, 8, 7, 2, 14, 15, 12, 10, 0, 6, 9, 1, 4, 11, 13, 3],
                [3, 6, 15, 14, 7, 9, 11, 10, 2, 1, 13, 5, 0, 12, 8, 4],
                [4, 8, 13, 1, 11, 7, 12, 10, 2, 3, 0, 14, 6, 5, 9, 15],
                [3, 12, 0, 11, 10, 5, 7, 14, 6, 13, 2, 15, 8, 9, 4, 1],
            ];
            for test in test_set.iter() {
                let is_solvable = is_solvable(test)?;
                assert_eq!(is_solvable, true);
            }
        }

        Ok(())
    }

    #[test]
    fn move_tile_should_generate_correct_arr() -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let mut arr = shuffle_arr(&mut rng)?;

        for _ in 0..10_000 {
            assert_eq!(is_solvable(&arr)?, true);

            let random_number = rng.gen_range(0, 4);
            let operation = match random_number {
                0 => Operation::UP,
                1 => Operation::DOWN,
                2 => Operation::LEFT,
                3 => Operation::RIGHT,
                _ => Operation::UP,
            };

            arr = move_tile(&arr, operation)?;
        }

        Ok(())
    }
}
