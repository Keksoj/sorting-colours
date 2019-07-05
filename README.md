# Sorting colours

A sorting exercise with a colourfull display.

## The purpose

Is to learn algorithmics. I first learned to implement quicksort in rust, then heapsort. But since arranging lists of numbers is a rather non-impressive deed, I decided to give it a little colour (a friend of mine is british, hence the spelling).

## A bit about heapsort

Heapsort implies to first build a *binary heap*, a kind of binary tree in which each child (in our case) is smaller than its parent. This process is called heapify.  
Then we swap the first element (the largest in the array) with the last one, which has two consequences :

1. The array has its largest element at the end - sorting has begun
2. The rules of the binary heap are violated, since its top element is not the biggest.

We have to compare it to its children and perform the swap if necessary, all the way down. This process has several name, I like *trickle down*.  
Then swap the new first element (which is the *second* largest of the array) with the *second to last* one, which will trickle down, and so on, and so forth.

## About the colours

I discovered Termion's [rainbow example](https://github.com/redox-os/termion/blob/master/examples/rainbow.rs) and found out a nice way of displaying RGB colours in a terminal:

```rust
write!(
    stdout,
    "{} ", // notice the space here
    termion::color::Bg(termion::color::Rgb(red, green, blue))
  ).unwrap();
```

So what I did is build three vectors of bytes like so:

```rust
pub struct Rainbow {
    rgb: Vec<Vec<u8>>,
}
```

fill every three to make it a range from 0 to 255. When displayed together, they show a smooth transition from black (0, 0, 0) to white (255, 255, 255).

What the program does then is : scramble the vector of the red values, scramble the green, scramble the blue. Then heapify each colour in that order, and heapsort them as well. Every little step is displayed on screen.

# How to use the program

Make sure to [install](https://www.rust-lang.org/learn/get-started) the Rust programming language and learn with its [excellent book](https://doc.rust-lang.org/book/).

Then clone the repository:

```sh
git clone https://github.com/Keksoj/sorting-colours.git
```

Change directory and run with cargo

```sh
cd sorting-colours
cargo run
```
