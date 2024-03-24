# AstroRust

![Version 0.0.1](https://img.shields.io/badge/version-0.0.1-blue)

An astrodynamics Rust library.

# Build Instructions
These instructions assume that Rust and Cargo are installed on your system. 

To set up this project, follow these steps:
1. Clone the repository:
    ```bash
    git clone https://github.com/ellie-sleightholm/astrorust.git
    cd astrorust
    ```
2. To use the AstroRust crate:
    ```bash
    cargo build
    ```
This will build everything you need to use astrorust. 

3. Running,
    ```bash
    cargo run --example time
    ```

will execute the time example. 

Likewise, we can take a TAI calendar and format it as a string:
```rust
let tai_cal_to_string = from_tai_calendar_to_gregorian(2000, 1, 1, 0, 0.0, 0.0);
assert_eq!(tai_cal_to_string, "2000-01-01T00:00:00.000");
```
