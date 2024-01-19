# Binary Search Algorithm in Rust

This repository contains a simple implementation of the binary search algorithm in Rust, showcasing its functionality and providing a basic understanding of how the algorithm works.

If you don't want to look at the code or you don't have rust compiler installed and just wants to see what the program does go [Here]() for the pre-compiled binaries

## Table of Contents

- [Introduction](#introduction)
- [How to Use](#how-to-use)
- [Code Explanation](#code-explanation)
- [Benchmarking](#benchmarking)
- [Press Any Key to Exit](#press-any-key-to-exit)

## Introduction

Binary search is an efficient algorithm used to find a specific value in a sorted array. This implementation demonstrates the binary search process with a visual aid, simulating real-time updates on the terminal during the search.

## How to Use

1. Clone the repository:

   ```bash
   git clone https://github.com/CollinEdward/Rust-Binary-Search.git
   cd Rust-Binary-Search
   ```

2. Run the Rust program:

   ```bash
   cargo run
   ```

3. Follow the on-screen instructions, and observe the binary search algorithm in action.

## Code Explanation

The main Rust program (`main.rs`) contains two functions: `binary_search` and `main`. The `binary_search` function performs the binary search on the provided sorted array, while the `main` function initializes the sorted array, specifies the target, and calls the `binary_search` function.

### Binary Search Function

```rust
fn binary_search(
    result: Result<usize, usize>,
    mut low: usize,
    mut high: usize,
    target: i32,
    array1: &[i32],
) {
    // ... (see code comments for detailed explanation)
}
```

The `binary_search` function takes parameters such as the search result, search boundaries (`low` and `high`), target value, and the sorted array (`array1`). It prints real-time updates about the search process, providing insights into the algorithm's steps.

### Main Function

```rust
fn main() {
    // ... (see code comments for detailed explanation)
}
```

The `main` function initializes the sorted array, specifies the target value, and calls the `binary_search` function. It also includes a simple benchmarking section to measure the elapsed time of the algorithm.

## Benchmarking

The program includes a benchmarking section that measures the elapsed time of the binary search algorithm using Rust's `std::time::Instant` module.

## Press Any Key to Exit

The program prompts the user to press any key to exit, allowing you to observe the results before terminating the program.

Feel free to explore and modify the code to enhance your understanding of the binary search algorithm in Rust.
