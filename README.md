# Depth-First Traversal of a Binary Tree in Rust

This project showcases a simple binary tree implementation in Rust and demonstrates a depth-first search traversal using a stack. The code includes:

1. A `Node<T>` struct representing each tree node (with optional left and right children).
2. A `depth_first_value` function that performs a depth-first traversal and returns the values of each node in the order they were visited.

## Overview

### What Does the Code Do?

1. **Builds a small binary tree** with the following structure:

      a
     / \
    b   c
   / \   \
  d   e   f

2. **Performs a depth-first search** (DFS) traversal on the tree using a stack.
3. **Prints the traversal order** to the console.

### Data Structure Details

- **`Node<T>` struct**  
Each node holds a string value (`val`), optional left and right child nodes (wrapped in a `Box` so they live on the heap), and a PhantomData field (`phantom`) which is a zero-sized type used here for illustrative purposes (it doesn't affect runtime behavior).

```rust
#[derive(Debug, Clone)]
struct Node<T> {
   val: String,
   left: Option<Box<Node<T>>>,
   right: Option<Box<Node<T>>>,
   phantom: std::marker::PhantomData<T>,
}
```
### Running the Code
1. Install Rust if you haven't already.
2. Make sure cargo is available on your system.
3. Clone or copy the code into a new Rust project or main.rs file.

Build and run:
cargo run
