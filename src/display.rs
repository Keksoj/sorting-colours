use rand::Rng;
use std::io::{stdout, Write};
use std::{thread, time};

// This struct contains all three vectors of RGB values
#[derive(Debug)]
pub struct Rainbow<W: Write> {
    rgb: Vec<Vec<u8>>,
    stdout: W,
    speed: u64,
}

impl<W: Write> Rainbow<W> {
    pub fn new(stdout: W) -> Rainbow<W> {
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

        let speed: u64 = 30;

        Rainbow { rgb, stdout, speed }
    }

    pub fn show(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        for index in 0..255 {
            // new line every 16 characters
            if index % 16 == 0 {
                write!(self.stdout, "\n\r").unwrap();
            }
            write!(
                self.stdout,
                "{}  ", // there are two spaces here !
                termion::color::Bg(termion::color::Rgb(
                    self.rgb[0][index],
                    self.rgb[1][index],
                    self.rgb[2][index]
                )),
            )
            .unwrap();
        }
        writeln!(self.stdout, "{}.", termion::style::Reset).unwrap();
    }

    pub fn scramble(&mut self, color: usize) {
        for n in 0..1000 {
            let random_index: usize = rand::thread_rng().gen_range(0, 128);
            let random_other: usize = rand::thread_rng().gen_range(128, self.rgb[color].len());
            if n % 100 == 0 {
                wait(self.speed);
                self.show();
            }
            self.rgb[color].swap(random_index, random_other);
        }
    }

    // the heapify function takes a rainbow.rgb[color] vector and constructs
    // a binary heap in place, iterating and trickling up each element
    pub fn heapify(&mut self, color: usize) {
        for i in 1..(self.rgb[color].len() - 1) {
            let mut index = i;
            loop {
                // find the parent of the element
                let parent_index = find_parent(index);
                // compare element with parent, swap them if necessary.
                if self.rgb[color][index] > self.rgb[color][parent_index] {
                    self.rgb[color].swap(index, parent_index);
                    // the next iteration of the loop will handle the parent
                    index = parent_index;
                    // display and wait a bit (for display purpose)
                    self.show();
                    wait(self.speed);
                } else {
                    break;
                }
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
    pub fn heapsort(&mut self, color: usize) {
        let mut last_index = self.rgb[color].len() - 1;

        loop {
            // swap the ends of the range we work on
            self.rgb[color].swap(0, last_index);

            // Trickle down the element of index 0
            let mut i: usize = 0;
            loop {
                // find the indexes of the left and right children
                let left_child_index = i * 2 + 1;
                let right_child_index = i * 2 + 2;
                // break if we reached the limit of the studied range
                if left_child_index >= last_index || right_child_index >= last_index {
                    break;
                }

                // Display and wait
                self.show();
                wait(self.speed);

                // Compare with the children, swap if necessary
                // case one: the left child is larger
                if self.rgb[color][left_child_index] > self.rgb[color][right_child_index] {
                    if self.rgb[color][i] < self.rgb[color][left_child_index] {
                        self.rgb[color].swap(i, left_child_index);
                        i = left_child_index;
                    } else {
                        break;
                    }
                // case two: the right child is larger
                } else {
                    if self.rgb[color][i] < self.rgb[color][right_child_index] {
                        self.rgb[color].swap(i, right_child_index);
                        i = right_child_index;
                    } else {
                        break;
                    }
                }
            }
            if last_index == 0 {
                break;
            }
            last_index -= 1;
        }
    }

    pub fn quicksort(&mut self, color: usize) {
        // To keep track of the indexes of the pivots used
        let mut pivots: Vec<usize> = Vec::new();
        // set the first pivot and index to start comparing to
        let mut pivot_i = self.rgb[color].len() - 1;
        let mut index = 0;
        // sort around the pivot. This will iterate over all the values left of it
        // and move them to the pivot's right, until the "iteration" reaches it.
        while index < pivot_i {
            // Compare the element with the pivot. If larger, put it to the pivot's
            // right. This will shift everything to the left, so the next studied
            // element will "fall in place", ready for the next comparison.
            if self.rgb[color][index] > self.rgb[color][pivot_i] {
                // Remove the element and put it into the variable 'swaped'
                let swaped = self.rgb[color].remove(index);
                // Insert the swaped element at the pivot's index (to its right)
                self.rgb[color].insert(pivot_i, swaped);
                // We have to keep track of the pivot, it shifted left
                pivot_i -= 1;
                wait(self.speed);

                self.show();
            // if the element is smaller, go check the next
            } else {
                self.show();
                index += 1;
            }
        }
        pivots.push(pivot_i);
        self.show();
        loop {
            // We're done when we have as much pivots as items
            if pivots.len() == self.rgb[color].len() {
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
                    if self.rgb[color][start_index] > self.rgb[color][new_pivot_index] {
                        let swaped = self.rgb[color].remove(start_index);
                        self.rgb[color].insert(new_pivot_index, swaped);
                        new_pivot_index -= 1;
                        self.show();
                        wait(self.speed);
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
            self.show();
            // Sort the last range IF the last item is not already counted as a pivot
            let mut last_pivot_index: usize = self.rgb[color].len() - 1;
            if !pivots.contains(&last_pivot_index) {
                while start_index < last_pivot_index {
                    if self.rgb[color][start_index] > self.rgb[color][last_pivot_index] {
                        let swaped = self.rgb[color].remove(start_index);
                        self.rgb[color].insert(last_pivot_index, swaped);
                        last_pivot_index -= 1;
                        wait(self.speed);
                        self.show();
                    } else {
                        start_index += 1;
                    }
                }
                pivots.push(last_pivot_index);
            }
        }
        self.show();
    }
}

pub fn wait(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
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
