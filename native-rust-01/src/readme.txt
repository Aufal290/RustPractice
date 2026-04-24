# Basic Data Types and Logic Practice in Rust

This repository contains a simple program designed to practice and demonstrate the fundamental components of the Rust programming language. The objective is to implement a basic logic flow using various data types and functional structures.

## 🚀 Key Learning Points

### 1. Scalar Data Types
* **Integers (`i32`)**: Managing numerical values for mathematical calculations and pricing.
* **Booleans (`bool`)**: Handling status verification and conditional flags.

### 2. Compound Data Types
* **Arrays**: Managing fixed-size lists of uniform data types (e.g., price lists).
* **Tuples**: Grouping different data types into a single container to represent complex entities (e.g., product details).

### 3. Core Concepts
* **Data Unpacking (Destructuring)**: Utilizing destructuring techniques to extract values from tuples into separate variables for cleaner and more readable code.
* **Control Flow**: 
    * Iterating through data collections using `for` loops.
    * Implementing conditional logic with `if-else` blocks to categorize data based on specific criteria.
* **Functional Programming**: Decoupling program logic into reusable functions, complete with parameters and return values.

## ⚙️ Program Flow
1. **Initialization**: The program sets up a collection of numerical data and a specific product package (tuple).
2. **Processing**: It iterates through the price list to identify specific price thresholds (e.g., categorizing expensive items).
3. **Validation**: It verifies stock availability through a dedicated boolean function.
4. **Calculation**: It performs percentage-based calculations to determine the final discounted price of an item.