# Getting Started with Rust on Linux

A practical starter guide covering installation, uninstallation, version checks, core syntax, data types, functions, loops, and Cargo package management on Linux.

---

## Table of Contents

1. [Install Rust on Linux](#install-rust-on-linux)
2. [Check Rust & Cargo Versions](#check-rust--cargo-versions)
3. [Uninstall Rust on Linux](#uninstall-rust-on-linux)
4. [Hello World & Project Structure](#hello-world--project-structure)
5. [Basic Syntax](#basic-syntax)
6. [Variables & Mutability](#variables--mutability)
7. [Data Types](#data-types)
8. [Functions](#functions)
9. [Loops & Control Flow](#loops--control-flow)
10. [Package Management (Cargo & Crates)](#package-management-cargo--crates)

---

## Install Rust on Linux

### 1. Install Build Tools & `curl`

Rust requires a C compiler/linker on Linux to link compiled binaries:

```bash
# Debian / Ubuntu / Linux Mint
sudo apt update && sudo apt install -y curl build-essential

# Fedora / RHEL
sudo dnf install -y curl gcc make

# Arch Linux
sudo pacman -Syu --needed curl base-devel
```

### 2. Run the Official `rustup` Installer

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

Press **`1`** (or `Enter`) to select the standard installation.

### 3. Load Rust into Your Current Shell

```bash
source "$HOME/.cargo/env"
```

---

## Check Rust & Cargo Versions

Verify that the compiler (`rustc`), package manager (`cargo`), and toolchain manager (`rustup`) are installed and in your `PATH`:

```bash
rustc --version
cargo --version
rustup --version
```

To update Rust to the latest release later:

```bash
rustup update
```

---

## Uninstall Rust on Linux

To completely remove Rust, `rustup`, Cargo, and the changes made to your shell profile:

```bash
rustup self uninstall
```

Confirm when prompted. If you want to verify no leftover directories remain, check `~/.rustup` and `~/.cargo`.

---

## Hello World & Project Structure

Create and run a new binary project with Cargo:

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

Generated layout:

- `Cargo.toml` — Project metadata and external dependencies (`[dependencies]`).
- `Cargo.lock` — Exact resolved versions of dependencies (managed automatically).
- `src/main.rs` — Main application entry point (`fn main()`).
- `target/` — Compiled build artifacts and binaries (safe to delete; rebuilt by Cargo).

Example `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Basic Syntax

- **Statements end with semicolons (`;`)**, while **expressions evaluate to a value** without a trailing semicolon.
- **Comments** use `//` for single lines and `/* ... */` for block comments.
- **`println!`** and **`vec!`** end with `!` because they are **macros**, not regular functions.
- **String formatting** uses `{}` placeholders (`println!("x = {}", x);`).

```rust
fn main() {
    // Line comment
    /* Block comment */

    let sum = {
        let a = 3;
        let b = 4;
        a + b // expression (no semicolon) -> returns 7
    };

    println!("Sum is: {sum}");
}
```

---

## Variables & Mutability

In Rust, variables are **immutable by default**. Add `mut` to allow reassignment:

```rust
fn main() {
    let name = "Ashraful"; // immutable
    let mut score = 10;    // mutable
    score += 5;

    // Shadowing: redeclare with `let` (can even change type)
    let spaces = "   ";
    let spaces = spaces.len();

    // Constants: always immutable, type annotation required
    const MAX_POINTS: u32 = 100_000;

    println!("{name} has {score} points, spaces={spaces}, max={MAX_POINTS}");
}
```

---

## Data Types

### Scalar Types

| Category | Types | Examples |
| :--- | :--- | :--- |
| **Signed Integers** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `let x: i32 = -42;` |
| **Unsigned Integers** | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `let count: usize = 10;` |
| **Floating-Point** | `f32`, `f64` (default) | `let pi: f64 = 3.14159;` |
| **Boolean** | `bool` | `let is_ready: bool = true;` |
| **Character** | `char` (4-byte Unicode scalar) | `let grade: char = 'A';` |

### Compound Types

```rust
fn main() {
    // Tuple: fixed length, mixed types
    let person: (&str, i32, bool) = ("Ada", 28, true);
    let (user_name, age, active) = person;
    println!("{user_name}, {age}, active={active}");

    // Array: fixed length, same type (lives on stack unless boxed)
    let numbers: [i32; 4] = [10, 20, 30, 40];
    println!("First element: {}", numbers[0]);

    // Vector: growable heap-allocated list
    let mut items = vec![1, 2, 3];
    items.push(4);
    println!("Vector: {:?}", items);

    // String (`String` is owned/growable; `&str` is a string slice)
    let mut greeting = String::from("Hello");
    greeting.push_str(", Rust!");
    println!("{greeting}");
}
```

---

## Functions

Functions are declared with `fn`. Parameters require type annotations, and return types use `->`. The final expression in a function body is returned automatically without needing `return`.

```rust
fn main() {
    greet("Linux");
    let total = add(7, 8);
    println!("7 + 8 = {total}");
}

fn greet(target: &str) {
    println!("Welcome, {target}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Loops & Control Flow

Rust provides three loop constructs: `loop`, `while`, and `for`.

```rust
fn main() {
    // 1. `loop` (infinite loop until `break`; can return a value)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // returns 10
        }
    };
    println!("Loop result: {result}");

    // 2. `while` (runs while condition is true)
    let mut n = 3;
    while n > 0 {
        println!("Countdown: {n}");
        n -= 1;
    }

    // 3. `for` (iterates over ranges or collections)
    for i in 1..=5 {
        println!("Inclusive range item: {i}");
    }

    let fruits = ["apple", "banana", "mango"];
    for fruit in fruits {
        println!("Fruit: {fruit}");
    }
}
```

---

## Package Management (Cargo & Crates)

External packages in Rust are called **crates**, hosted on [crates.io](https://crates.io/). Downloaded crate caches live in `~/.cargo/registry/`, while project-specific compiled artifacts live in `./target/`.

```bash
# Add a crate from crates.io (updates Cargo.toml and Cargo.lock)
cargo add rand

# Build in debug mode (fast compile)
cargo build

# Build in release mode (optimized for performance -> target/release/)
cargo build --release

# Run tests
cargo test

# Clean build artifacts in ./target
cargo clean
```

Example using `rand` in `src/main.rs`:

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let roll = rng.random_range(1..=6);
    println!("You rolled a {roll}!");
}
```