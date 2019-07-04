// 2019-06-27

use rand::Rng;
// use std::{thread, time};

// First let's get a vector of numbers from 1 to 2000
pub fn scramble(vector: &mut Vec<u8>) {
    for _n in 0..1000 {
        let random_index: usize = rand::thread_rng().gen_range(0, 128);
        let random_other: usize = rand::thread_rng().gen_range(128, vector.len());
        vector.swap(random_index, random_other);
    }
}
