// 2019-06-27
// Let's dive into sorting algorithms.

mod quicksort;
mod rainbow;
mod scramble;
use rainbow::Tricolors;
use std::io::{stdin, stdout, Write};
// use termion::event::Key;
// use termion::input::TermRead;
use std::{thread, time};
use termion::raw::IntoRawMode;

// let mut vecteur = new_scrambled_vector(limit);
// let sorted = naive_sort(&mut vecteur);
// println!("{:?}", sorted);

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut tricolors = Tricolors::new(stdin, stdout.lock());

    tricolors.show();
    thread::sleep(time::Duration::from_secs(2));

    tricolors.scramble();

    tricolors.heapify();

    tricolors.heapsort();
}

// testing the heapify function

// let mut vector: Vec<u8> = Vec::new();
// for n in 0..255 {
//     vector.push(n);
// }
// scramble::scramble(&mut vector);
// println!("{:?}", vector);
// heapify(&mut vector);
// println!("{:?}", vector);
