# Rust Basic Logic & Data Types Practice

This repository contains a practical implementation of fundamental Rust concepts. The project focuses on a simple retail logic: calculating product totals, checking for bonuses, and applying multiple discount scenarios using loops and functions.

## 🚀 Key Concepts Covered

### 1. Data Structures
* **Tuples**: Used to store heterogeneous product data `(name, price, stock)`.
* **Arrays**: Used to store a fixed list of available discount percentages `[5, 10, 15]`.
* **Destructuring**: Breaking down the product tuple into individual variables for easier access.

### 2. Control Flow & Logic
* **Functions with Return Types**: 
    * `BonusSticker`: Returns a `bool` based on a price threshold.
    * `apply_discount`: Returns an `i32` after performing mathematical calculations.
* **For Loops**: Iterating through the discount array to calculate and display various price scenarios.
* **Conditional Statements**: Using `if` blocks to trigger specific events like bonus rewards.

### 3. Rust Specifics
* **Immutability vs Mutability**: Handling the `mut` keyword to allow variable updates within a loop.
* **Expressions**: Utilizing Rust's expression-based return in functions (omitting the `return` keyword and semicolon).
* **Integer Arithmetic**: Managing safe mathematical operations within the `i32` data type.

## ⚙️ Program Flow
1.  **Initialize**: Define a product tuple and an array of discounts.
2.  **Verify**: Check if the total price qualifies for a "Bonus Sticker" using a dedicated function.
3.  **Process**: Loop through the discount array.
4.  **Calculate**: In each iteration, the `total_price` is updated by the `apply_discount` function, demonstrating how values can be mutated and reassigned.
5.  **Output**: Print the updated price for each discount tier.