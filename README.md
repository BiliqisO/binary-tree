# Binary Tree Traversal & Utility Functions in Rust

This project demonstrates how to build a simple binary tree in Rust and then perform various operations on it. These operations include:

- **Depth-First Search** (DFS)
- **Breadth-First Search** (BFS)
- **Checking for a value** in the tree (both breadth-first and recursive approaches)
- **Computing the sum** of all node values in the tree

## Table of Contents
- [Overview](#overview)
- [Data Structure](#data-structure)
- [Functions](#functions)
  - [Depth-First Search](#depth-first-search)
  - [Breadth-First Search](#breadth-first-search)
  - [Tree Includes (Breadth-First)](#tree-includes-breadth-first)
  - [Tree Includes (Recursive)](#tree-includes-recursive)
  - [Tree Sum](#tree-sum)
- [Usage](#usage)
- [Output](#output)
- [Notes](#notes)

---

## Overview

In `main()`, we construct a small integer-based binary tree and then call several functions that operate on the tree:

```text
         3
        / \
      11   4
     / \    \
    4   2    1

```

After building this tree, the code demonstrates:

1. **Depth-first traversal of the tree, returning a list of visited node values.
2. **Breadth-first traversal of the tree, also returning a list of visited node values.
3. **tree_includes_breadth: A breadth-first search (BFS) approach that checks if a given string-based value is present in the tree (by converting each node’s value to a string and comparing).
4. **tree_includes_recursive: A recursive approach that checks if a given value (again, as a string) is present in the tree.
5. **tree_sum: Sums up all node values in the tree.

### Data Structure Details

- **`Node<T>` struct**  
1. **val: T: The node’s value (generic type).
2. **left: An Option-wrapped Box pointing to the left child node.
3. **right: Same as above, but for the right child node.
We use boxes (Box) because this is a potentially recursive data structure stored on the heap.
```rust
#[derive(Debug, Clone)]
struct Node<T> {
   val: T,
   left: Option<Box<Node<T>>>,
   right: Option<Box<Node<T>>>
}
```
### Running the Code
1. Install Rust if you haven't already.
2. Make sure cargo is available on your system.
3. Clone or copy the code into a new Rust project or main.rs file.

Build and run:
cargo run
