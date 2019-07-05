// inspired by termion's rainbow and games examples
use crate::scramble::scramble;
use std::cmp::Ordering;
use std::io::{Read, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, color, cursor, style, terminal_size};

#[derive(Debug)]
pub struct Rainbow<R, W: Write> {
    stdout: W,
    stdin: R,
    rgb: Vec<Vec<u8>>,
    x: u16,
    y: u16,
}

impl<R: Read, W: Write> Rainbow<R, W> {
    pub fn new(stdin: R, stdout: W) -> Rainbow<R, RawTerminal<W>> {
        // calculate from where we will draw the game
        let x = (terminal_size().unwrap().0 / 2) + 10;
        let y = (terminal_size().unwrap().1 / 2);

        // create the rgb vectors
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

        Rainbow {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin,
            rgb,
            x,
            y,
        }
    }

    pub fn run(&mut self) {
        // scramble all three colors
        self.show();
        for i in 0..3 {
            scramble(&mut self.rgb[i]);
            thread::sleep(time::Duration::from_secs(4));
            self.show();
        }

        // heapify all three colors
        for i in 0..3 {
            heapify_vector(&mut self.rgb[i]);
            thread::sleep(time::Duration::from_secs(4));
            self.show();
        }
        // heapsort all three colors
        for i in 0..3 {
            heapsort_vector(&mut self.rgb[i]);
            thread::sleep(time::Duration::from_secs(4));
            self.show();
        }
    }


    pub fn show(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(self.x, 1),
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
                "{}  ", // there is a space here !
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
}

pub fn heapsort_vector(vector: &mut Vec<u8>) {
    // find the index of the last element
    let mut last_index = vector.len() - 1;

    // Swap that last element and tricke it down
    loop {
        // swap the ends of the range we work on
        vector.swap(0, last_index);

        // trickle down
        let mut i: usize = 0;
        loop {
            // find the indexes of the left and right children
            let l = i * 2 + 1;
            let r = i * 2 + 2;
            if l >= last_index || r >= last_index {
                break;
            }

            if vector[l] > vector[r] {
                if vector[i] < vector[l] {
                    vector.swap(i, l);
                    i = l;
                } else {
                    break;
                }
            // case two: bigger child is right
            } else {
                if vector[i] < vector[r] {
                    vector.swap(i, r);
                    i = r;
                } else {
                    break;
                }
            }
        }
        // we're done when we reached index 1
        if last_index == 0 {
            break;
        }
        // reduce the range
        last_index = last_index - 1;
    }
}

// the heapify function takes a scrambled vector, pushes its element into a new
// binary heap, at the bottom, and make it trickle up
pub fn heapify_vector(vector: &mut Vec<u8>) {
    match vector[1].cmp(&vector[0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => vector.swap(1, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }

    // add a third element and repeat the comparison with the ROOT
    match vector[2].cmp(&vector[0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => vector.swap(2, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }

    for i in 3..(vector.len() - 1) {
        let mut index = i;
        // trickle it up
        loop {
            if index == 0 {
                break;
            }
            let p_i = find_parent(index);

            // if greater than its parent, swap them. If not, we're done
            if vector[index] > vector[p_i] {
                vector.swap(index, p_i);
                index = p_i;
            } else {
                break;
            }
        }
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
