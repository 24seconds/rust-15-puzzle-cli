use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::error::Error;

pub fn shuffle_arr(rng: &mut ThreadRng) -> [u16; 16] {
    let mut arr = [0; 16];

    (0..16).into_iter().enumerate().for_each(|args| {
        let (index, number) = args;

        arr[index] = number;
    });

    arr.shuffle(rng);

    arr
}

