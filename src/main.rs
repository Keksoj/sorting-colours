// 2019-06-27
// Let's dive into sorting algorithms.

use rand::Rng;
// use std::cmp::Ordering;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::color;
use termion::raw::IntoRawMode;

mod display;
use display::Rainbow;

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();

    let mut rainbow = Rainbow::new(stdout.lock());

    for i in 0..3 {
        rainbow.scramble(i);
        wait(2000);
        rainbow.show();
    }

    // QUICKSORT
    for i in 0..3 {
        rainbow.quicksort(i);
        wait(2000);
        rainbow.show();
    }
    for i in 0..3 {
        rainbow.scramble(i);
        wait(2000);
        rainbow.show();
    }
    // heapify
    for i in 0..3 {
        rainbow.heapify(i);
        wait(1000);
        rainbow.show();
    }
    // heapsort
    for i in 0..3 {
        rainbow.heapsort(i);
        wait(1000);
        rainbow.show();
    }
}

fn wait(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}