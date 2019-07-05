// 2019-06-27
// Let's dive into sorting algorithms.
use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style, terminal_size};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut rainbow = Rainbow::new();

    // scramble all three colors
    for i in 0..3 {
        scramble(&mut stdout, &mut rainbow, i);
        thread::sleep(time::Duration::from_secs(2));
        show(&mut stdout, &rainbow);
    }
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

#[derive(Debug)]
pub struct Rainbow {
    rgb: Vec<Vec<u8>>,
}

impl Rainbow {
    pub fn new() -> Rainbow {
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

        Rainbow { rgb }
    }
}

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
    for _n in 0..1000 {
        let random_index: usize = rand::thread_rng().gen_range(0, 128);
        let random_other: usize = rand::thread_rng().gen_range(128, rainbow.rgb[color].len());
        thread::sleep(time::Duration::from_millis(20));
        show(&mut stdout, &rainbow);

        rainbow.rgb[color].swap(random_index, random_other);
    }
}

// the heapify function takes a scrambled vector, pushes its element into a new
// binary heap, at the bottom, and make it trickle up
pub fn heapify<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    match rainbow.rgb[color][1].cmp(&rainbow.rgb[color][0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => rainbow.rgb[color].swap(1, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }

    // add a third element and repeat the comparison with the ROOT
    match rainbow.rgb[color][2].cmp(&rainbow.rgb[color][0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => rainbow.rgb[color].swap(2, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }

    for i in 3..(rainbow.rgb[color].len() - 1) {
        let mut index = i;
        // trickle it up
        loop {
            if index == 0 {
                break;
            }
            let p_i = find_parent(index);

            // if greater than its parent, swap them. If not, we're done
            if rainbow.rgb[color][index] > rainbow.rgb[color][p_i] {
                rainbow.rgb[color].swap(index, p_i);
                index = p_i;
                show(&mut stdout, &rainbow);
                thread::sleep(time::Duration::from_millis(50));

            } else {
                break;
            }
        }
    }
}

pub fn heapsort<W: Write>(mut stdout: W, rainbow: &mut Rainbow, color: usize) {
    // find the index of the last element
    let mut last_index = rainbow.rgb[color].len() - 1;

    // Swap that last element and tricke it down
    loop {
        // swap the ends of the range we work on
        rainbow.rgb[color].swap(0, last_index);

        // trickle down
        let mut i: usize = 0;
        loop {
            // find the indexes of the left and right children
            let l = i * 2 + 1;
            let r = i * 2 + 2;
            if l >= last_index || r >= last_index {
                break;
            }
            show(&mut stdout, &rainbow);
            thread::sleep(time::Duration::from_millis(50));
            if rainbow.rgb[color][l] > rainbow.rgb[color][r] {
                if rainbow.rgb[color][i] < rainbow.rgb[color][l] {
                    rainbow.rgb[color].swap(i, l);
                    i = l;
                } else {
                    break;
                }
            // case two: bigger child is right
            } else {
                if rainbow.rgb[color][i] < rainbow.rgb[color][r] {
                    rainbow.rgb[color].swap(i, r);
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
