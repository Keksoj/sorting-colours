use std::cmp::Ordering;
use std::{thread, time};

// the heapify function takes a scrambled vector, pushes its element into a new
// binary heap, at the bottom, and make it trickle up
pub fn heapify(scrambled: &mut Vec<u32>) -> Vec<u32> {
    let mut max_heap: Vec<u32> = Vec::new();

    // add one element to the max_heap
    max_heap.push(scrambled.pop().unwrap());
    println!("{:?}", max_heap);

    // add a second element
    max_heap.push(scrambled.pop().unwrap());
    println!("{:?}", max_heap);

    // compare the new element (greater index) to the root (index 0)
    match max_heap[1].cmp(&max_heap[0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => max_heap.swap(1, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }
    println!("{:?}", max_heap);

    // add a third element and repeat the comparison with the ROOT
    max_heap.push(scrambled.pop().unwrap());
    println!("{:?}", max_heap);
    match max_heap[2].cmp(&max_heap[0]) {
        // if the new element is greater, swap it with the root
        Ordering::Greater => max_heap.swap(2, 0),
        Ordering::Less => {}
        Ordering::Equal => {}
    }

    // insert all the other values and trickle them up
    loop {
        // if the list from which we take the new numbers is empty, our job is done
        if scrambled.is_empty() {
            break;
        }

        // Add one element from the scrambled list
        max_heap.push(scrambled.pop().unwrap());

        // get the index of this newest element
        let mut i = max_heap.len() - 1;

        // trickle it up
        loop {
            // break the loop before it reaches the root !
            if i == 0 {
                break;
            }
            let p_i = find_parent(i);

            // if greater than its parent, swap them. If not, we're done
            if max_heap[i] > max_heap[p_i] {
                max_heap.swap(i, p_i);
                i = p_i;
            } else {
                break;
            }
        }
    }
    println!("here is the max heap_______: {:?}", max_heap);
    max_heap
}

pub fn heapsort(max_heap: &mut Vec<u32>) {
    // find the index of the last element
    let mut last_index = max_heap.len() - 1;

    // Swap that last element and tricke it down
    loop {
        // swap the ends of the range we work on
        max_heap.swap(0, last_index);
        // the first element being the biggest, it is now at the end, sorted
        // println!("swapped the ends___________: {:?}", max_heap);

        // trickle down
        let mut i: usize = 0;
        loop {
            // find the indexes of the left and right children
            let l = i * 2 + 1;
            let r = i * 2 + 2;
            if l >= last_index || r >= last_index {
                break;
            }
            // thread::sleep(time::Duration::from_millis(50));

            // Compare the parent key with its children
            // Find the BIGGER child and swap the parent with it
            // case one : bigger child is left
            if max_heap[l] > max_heap[r] {
                if max_heap[i] < max_heap[l] {
                    max_heap.swap(i, l);
                    i = l;
                } else {
                    break;
                }
            // case two: bigger child is right
            } else {
                if max_heap[i] < max_heap[r] {
                    max_heap.swap(i, r);
                    i = r;
                } else {
                    break;
                }
            }

            println!(
                // "trickle-down to the _ index: {:?}", max_heap
            );
        }
        // we're done when we reached index 1
        if last_index == 0 {
            break;
        }
        // reduce the range
        last_index = last_index - 1;
    }
    println!("{:?}", max_heap);
}

// we had the thread panicking at 'attempt to subtract with overflow' for such
// a trivial thing as doing (i - 1) / 2  so we had to adapt.
// Wait. Who's "we"?
fn find_parent(i: usize) -> usize {
    i.wrapping_sub(1).wrapping_div(2)
}
