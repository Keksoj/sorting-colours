// 2019-06-27
// Let's dive into sorting algorithms.

use rand::Rng;
// use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use termion::color;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut rainbow = Rainbow::new();

    // // scramble the red
    // scramble(&mut stdout, &mut rainbow, 0);
    // // quicksort the red
    // quicksort(&mut stdout, &mut rainbow, 0);

    // scramble all three colors
    for i in 0..3 {
        scramble(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(2));
        show(&mut stdout, &rainbow);
    }
    // QUICKSORT
    for i in 0..3 {
        quicksort(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(1));
        show(&mut stdout, &rainbow);
    }

    // scramble all three colors
    for i in 0..3 {
        scramble(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(2));
        show(&mut stdout, &rainbow);
    }
    // HEAPSORT
    // heapify all three colors
    for i in 0..3 {
        heapify(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(1));
        show(&mut stdout, &rainbow);
    }
    // heapsort all three colors
    for i in 0..3 {
        heapsort(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(1));
        show(&mut stdout, &rainbow);
    }
}

// This struct contains all three vectors of RGB values
#[derive(Debug)]
pub struct Rainbow {
    rgb: Vec<Vec<u8>>,
}

impl Rainbow {
    // create the red, green and blue vectors, each of the shape [0, 1, 2, ..255]
    pub fn new() -> Rainbow {
        let mut red: Vec<u8> = Vec::new();
        let mut green: Vec<u8> = Vec::new();
        let mut blue: Vec<u8> = Vec::new();
        for n in 0..255 {
            red.push(n);
            green.push(n);
            blue.push(n);
        }
        let mut rgb: Vec<Vec<u8>> = Vec::new();
        rgb.push(red);
        rgb.push(green);
        rgb.push(blue);

        Rainbow { rgb }
    }
}

// This is Termion's way of displaying a square of colours.
pub fn show<W: Write>(stdout: &mut W, rainbow: &Rainbow) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();

    for index in 0..255 {
        // new line every 16 characters
        if index % 16 == 0 {
            write!(stdout, "\n\r").unwrap();
        }
        write!(
            stdout,
            "{}  ", // there are two spaces here !
            termion::color::Bg(termion::color::Rgb(
                rainbow.rgb[0][index],
                rainbow.rgb[1][index],
                rainbow.rgb[2][index]
            )),
        )
        .unwrap();
    }
    writeln!(stdout, "{}.", termion::style::Reset).unwrap();
}

pub fn scramble<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    // swap a random element of the left half with a random element of the right
    // half, reapeat a thousand times. Surely not optimal, but it works
    for n in 0..1000 {
        let random_index: usize = rand::thread_rng().gen_range(0, 128);
        let random_other: usize = rand::thread_rng().gen_range(128, rainbow.rgb[color].len());
        if n % 100 == 0 {
            thread::sleep(time::Duration::from_millis(200));
            show(&mut stdout, &rainbow);
        }

        rainbow.rgb[color].swap(random_index, random_other);
    }
}

////////////////////////////////////////////////////////////////////////////////
//       _   _   _____      _      ____    ____     ____    ____    _____     //
//      | | | | | ____|    / \    |  _ \  / ___|   / __ \  |  _ \  |_   _|    //
//      | |_| | |  _|     / _ \   | |_) | \___ \  | |  | | | |_) |   | |      //
//      |  _  | | |___   / ___ \  |  __/   ___) | | |__| | |  _ <    | |      //
//      |_| |_| |_____| /_/   \_\ |_|     |____/   \____/  |_| \_\   |_|      //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

// the heapify function takes a rainbow.rgb[color] vector and constructs a binary heap
// in place, iterating and trickling up each element
pub fn heapify<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    for i in 1..(rainbow.rgb[color].len() - 1) {
        let mut index = i;
        loop {
            // find the parent of the element
            let p_i = find_parent(index);

            // compare element with parent, swap them if necessary.
            if rainbow.rgb[color][index] > rainbow.rgb[color][p_i] {
                rainbow.rgb[color].swap(index, p_i);
                // the next iteration of the loop will handle the parent
                index = p_i;
                // display and wait a bit (for display purpose)
                show(&mut stdout, &rainbow);
                thread::sleep(time::Duration::from_millis(30));
            } else {
                break; // break if the parent was actually larger
            }
            // Break if we reached the to of the binary heap
            if index == 0 {
                break;
            }
        }
    }
}

// The heapsort function takes a max_heap (a binary heap where each element is
// greater than its children), swap the first element with the last.
// the largest element is now at the last position (sorting has begun)
// the new first element violates the property of the heap (it is too small)
// and we trickle down to its appropriate position.
// Repeat by swaping the new first element with the second to last
pub fn heapsort<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    // find the index of the last element
    let mut last_index = rainbow.rgb[color].len() - 1;

    loop {
        // swap the ends of the range we work on
        rainbow.rgb[color].swap(0, last_index);

        // Trickle down the element of index 0
        let mut i: usize = 0;
        loop {
            // find the indexes of the left and right children
            let l = i * 2 + 1;
            let r = i * 2 + 2;
            // break if we reached the limit of the studied range
            if l >= last_index || r >= last_index {
                break;
            }

            // Display and wait
            show(&mut stdout, &rainbow);
            thread::sleep(time::Duration::from_millis(30));

            // Compare with the children, swap if necessary
            // case one: larger child is the left one
            if rainbow.rgb[color][l] > rainbow.rgb[color][r] {
                if rainbow.rgb[color][i] < rainbow.rgb[color][l] {
                    rainbow.rgb[color].swap(i, l);
                    i = l;
                } else {
                    break;
                }
            // case two: larger child is the right one
            } else {
                if rainbow.rgb[color][i] < rainbow.rgb[color][r] {
                    rainbow.rgb[color].swap(i, r);
                    i = r;
                } else {
                    break;
                }
            }
        }
        // we're done when we reached index 0
        if last_index == 0 {
            break;
        }
        // last element is now sorted, reduce the range we work on
        last_index = last_index - 1;
    }
}

// we had the thread panicking at 'attempt to subtract with overflow' for such
// a trivial thing as doing (i - 1) / 2  so we had to adapt.
// This ugly code is how we avoid such messages:
// "the len is 255 but the index is 9223372036854775807"
// 'Wait. Who's "we"?'
// 'Oh, shut up.'
fn find_parent(i: usize) -> usize {
    match i {
        1 => 0,
        2 => 0,
        _ => i.wrapping_sub(1).wrapping_div(2),
    }
}

////////////////////////////////////////////////////////////////////////////////
//      ___    _   _   ___    ____   _  __  ____     ____    ____    _____    //
//     / _ \  | | | | |_ _|  / ___| | |/ / / ___|   / __ \  |  _ \  |_   _|   //
//    | | | | | | | |  | |  | |     | ' /  \___ \  | |  | | | |_) |   | |     //
//    | |_| | | |_| |  | |  | |___  | . \   ___) | | |__| | |  _ <    | |     //
//     \__\_\  \___/  |___|  \____| |_|\_\ |____/   \____/  |_| \_\   |_|     //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

// This code is redundant because I needed to use the show() method inside it

fn quicksort<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    // To keep track of the indexes of the pivots used
    let mut pivots: Vec<usize> = Vec::new();

    // set the first pivot and index to start comparing to
    let mut pivot_i = rainbow.rgb[color].len() - 1;
    let mut index = 0;

    // sort around the pivot. This will iterate over all the values left of it
    // and move them to the pivot's right, until the "iteration" reaches it.
    while index < pivot_i {
        // Compare the element with the pivot. If larger, put it to the pivot's
        // right. This will shift everything to the left, so the next studied
        // element will "fall in place", ready for the next comparison.
        if rainbow.rgb[color][index] > rainbow.rgb[color][pivot_i] {
            // Remove the element and put it into the variable 'swaped'
            let swaped = rainbow.rgb[color].remove(index);
            // Insert the swaped element at the pivot's index (to its right)
            rainbow.rgb[color].insert(pivot_i, swaped);
            // We have to keep track of the pivot, it shifted left
            pivot_i -= 1;
            thread::sleep(time::Duration::from_millis(50));
            show(&mut stdout, &rainbow);

        // if the element is smaller, go check the next
        } else {
            show(&mut stdout, &rainbow);
            index += 1;
        }
    }
    // Add the pivot to the list of pivots
    pivots.push(pivot_i);
    show(&mut stdout, &rainbow);

    loop {
        // We're done when we have as much pivots as items
        if pivots.len() == rainbow.rgb[color].len() {
            break;
        }

        // Create a temporary collection of pivots
        let mut temp_pivots: Vec<usize> = Vec::new();
        let mut start_index = 0;
        // Iterate over the pivots to sort the ranges between them
        for existing_pivot_index in pivots.iter() {
            // Check if the range is long enough to sort
            // first case: the pivots were next to each other
            if start_index == *existing_pivot_index {
                start_index += 1;
                continue;
            // second case: the pivots were one value appart
            } else if start_index + 1 == *existing_pivot_index {
                temp_pivots.push(start_index);
                start_index += 2;
                continue;
            }

            // sort the range
            // set the new pivot
            let mut new_pivot_index = *existing_pivot_index - 1;
            while start_index < new_pivot_index {
                if rainbow.rgb[color][start_index] > rainbow.rgb[color][new_pivot_index] {
                    let swaped = rainbow.rgb[color].remove(start_index);
                    rainbow.rgb[color].insert(new_pivot_index, swaped);
                    new_pivot_index -= 1;
                    show(&mut stdout, &rainbow);
                    thread::sleep(time::Duration::from_millis(50));
                } else {
                    start_index += 1;
                }
            }
            temp_pivots.push(new_pivot_index);
            // Prepare to study the next range
            start_index = existing_pivot_index + 1;
        }

        // merge the pivots vectors
        pivots.append(&mut temp_pivots);
        pivots.sort(); // this kinda defeats the whole purpose, does it?
        show(&mut stdout, &rainbow);

        // Sort the last range IF the last item is not already counted as a pivot
        let mut last_pivot_index: usize = rainbow.rgb[color].len() - 1;
        if !pivots.contains(&last_pivot_index) {
            while start_index < last_pivot_index {
                if rainbow.rgb[color][start_index] > rainbow.rgb[color][last_pivot_index] {
                    let swaped = rainbow.rgb[color].remove(start_index);
                    rainbow.rgb[color].insert(last_pivot_index, swaped);
                    last_pivot_index -= 1;
                    thread::sleep(time::Duration::from_millis(50));

                    show(&mut stdout, &rainbow);
                } else {
                    start_index += 1;
                }
            }
            pivots.push(last_pivot_index);
        }
    }

    show(&mut stdout, &rainbow);
}
