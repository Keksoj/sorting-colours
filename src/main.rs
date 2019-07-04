// 2019-06-27
// Let's dive into sorting algorithms.

use std::io;
mod scramble;
use scramble::new_scrambled_vector;
mod quicksort;
use quicksort::quicksort;
mod heapsort;
use heapsort::{heapify, heapsort};

// let mut vecteur = new_scrambled_vector(limit);
// let sorted = naive_sort(&mut vecteur);
// println!("{:?}", sorted);

fn main() {
    // println!("Hello ! What length should the scrambled list be? We'll quicksort it.");
    // let mut limit = String::new();
    // io::stdin()
    //     .read_line(&mut limit)
    //     .expect("failed to read lin");
    // let limit: u32 = limit.trim().parse().unwrap();
    //
    // let quick_vector = new_scrambled_vector(limit);
    // quicksort(quick_vector);

    println!("Cool! Another one! Heapsort!");
    let mut limit = String::new();
    io::stdin()
        .read_line(&mut limit)
        .expect("failed to read lin");
    let limit: u32 = limit.trim().parse().unwrap();

    let mut scrambled = new_scrambled_vector(limit);
    println!("{:?}", scrambled);
    let mut sorted = heapify(&mut scrambled);
    // println!("{:?}", sorted);

    heapsort(&mut sorted);
}
