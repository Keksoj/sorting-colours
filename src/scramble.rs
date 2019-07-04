// 2019-06-27

use rand::Rng;
// use std::{thread, time};

// First let's get a vector of numbers from 1 to 2000
pub fn new_scrambled_vector(limit: u32) -> Vec<u32> {
    let mut number: u32 = 0;
    let mut vector: Vec<u32> = Vec::new();
    while number <= (limit - 1) {
        vector.push(number);
        number += 1;
    }

    let mut scrambled: Vec<u32> = Vec::new();

    while vector.len() > 0 {
        let index: usize = rand::thread_rng().gen_range(0, vector.len());

        // Read from right to left. This is supposed to :
        // find the value of the vector at the random index
        // take it from the vector
        // push it to the scrambled one
        scrambled.push(vector.remove(index));
    }

    scrambled
}

// let's try and sort it very naively
// This algorithm compares each number with its neighbor, switch their position
// if needed, compares the next pair. Repeat n times, n being the length of the
// list
pub fn naive_sort(vector: &mut Vec<u32>) -> Vec<u32> {
    let start_time = std::time::SystemTime::now();

    for _n in 0..vector.len() {
        for index in 0..(vector.len() - 1) {
            if vector[index] > vector[index + 1] {
                let value = vector[index + 1];
                vector[index + 1] = vector[index];
                vector[index] = value;
                // thread::sleep(time::Duration::from_millis(50));
                // println!("{:?}", vector);
            }
        }
    }
    println!(
        "Sorting performed in {} ms",
        start_time.elapsed().unwrap().as_millis()
    );
    vector.to_vec()
}

#[cfg(test)]
mod tests {

    use crate::scramble::naive_sort;
    #[test]
    fn naive_sort_works() {
        let mut unsorted = [10, 7, 2, 8, 1, 9, 4, 6, 5, 3].to_vec();
        let sorted = naive_sort(&mut unsorted);

        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec());
    }
}
