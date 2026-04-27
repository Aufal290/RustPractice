# 🦀 Rust Memory Management: Ownership, Structs, and Borrowing

## 📖 Overview
This project serves as a technical deep-dive into Rust's core memory management principles. By building a simplified **Book Tracking System**, this lab explores how Rust ensures memory safety and prevents common bugs like data races and dangling pointers without a garbage collector.

## 🎯 Key Learning Objectives

### 1. Ownership Transfer (The "Move" Semantics)
In the `create_book()` function, I implemented **Ownership Transfer**.
* **Mechanism:** Data is initialized within a local scope inside a `struct`, then moved out of the function via the return value.
* **Outcome:** The original struct instance is dropped (cleared from the stack), but the heap data (the String) survives because its ownership is transferred to the caller in `main`.

### 2. Immutable Borrowing (`&T`)
The `show_book` function demonstrates **shared access** through immutable references.
* **Principle:** Passing `&String` allows the function to read the data without taking ownership.
* **Advantage:** The data remains accessible in the `main` scope for subsequent operations since it was never "given away," only "lent."

### 3. Mutable Borrowing (`&mut T`)
The `edit_edition` function utilizes **exclusive access** to modify data in-place.
* **Execution:** By using `&mut String`, the function gains "Editor" privileges.
* **Constraint:** Rust's borrow checker ensures that only one mutable reference exists at a time, preventing data races at compile time.
* **Operation:** I used `.push_str()` to append data directly to the existing memory allocation on the heap.

### 4. Memory Safety & Lifetime Guarantees
A crucial part of this exercise was understanding why **Dangling References** are impossible in Rust.
* **Prevention:** Rust prevents returning a reference (`&String`) to a value created inside a function because that value would be dropped at the end of the function's scope, leaving the reference pointing to invalid memory.

## 🛠️ Code Structure & Logic Flow

| Component | Responsibility | Rust Concept |
| :--- | :--- | :--- |
| `struct Book` | Data modeling for book attributes. | Custom Data Types |
| `create_book` | Heap allocation and ownership return. | Move Semantics |
| `show_book` | Read-only access to display titles. | Immutable Borrowing |
| `edit_edition` | In-place modification of strings. | Mutable Borrowing |

## 🧠 Final Reflection
Through this project, I've realized that Rust isn't just about syntax; it's about a strict contract between the developer and the compiler. By explicitly defining who "owns" the data and who is just "borrowing" it, Rust achieves C++ level performance with modern safety guarantees.