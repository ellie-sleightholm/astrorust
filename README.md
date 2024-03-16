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

# Examples 

### Example 1 - Conversion Between Timesystems
Let's define a time variable `tai_jd` which corresponds to a date in
Julian Day format for the TAI timesystem. 
```rust
let tai_jd = 2451545.0;
```
If we want to convert this Julian day to another timesystem, we can do
so by using the `convert_from_jd_to_jd` function. This converts from one Julian day (in a given timesystem) to another Julian day (in another timesystem). Inputting the Julian days with the timesystem you're converting from and to, respectively.
```rust
let tai_jd_to_utc_jd = convert_from_jd_to_jd(tai_jd, Timescale::TAI, Timescale::UTC);
assert_eq!(tai_jd_to_utc_jd, 2451544.99962963);
let tai_jd_to_tdb_jd = convert_from_jd_to_jd(tai_jd, Timescale::TAI, Timescale::TDB);
assert_eq!(tai_jd_to_tdb_jd, 2451545.000372499);
```

Suppose instead, you want to convert from a Julian date to a Gregorian string. The function `convert_from_jd_to_gregorian` allows you to do so:
```rust
let utc_greg = convert_from_jd_to_gregorian(2451545.0, Timescale::TAI, Timescale::UTC);
assert_eq!(utc_greg, "2000-01-01T11:59:28.000");
let tai_greg = convert_from_jd_to_gregorian(2451544.99962963, Timescale::UTC, Timescale::TAI);
assert_eq!(tai_greg, "2000-01-01T12:00:00.000");
```

If the converse is true, that is you wish to convert from a Gregorian string to a Julian date, you use the `convert_gregorian_to_jd` function:
```rust
let utc_jd = convert_gregorian_to_jd("2000-01-01T12:00:32.184", Timescale::TT, Timescale::UTC);
assert_eq!(utc_jd, 2451544.99962963);
let tai_jd = convert_gregorian_to_jd("2000-01-01T12:00:32.184", Timescale::TT, Timescale::TAI);
assert_eq!(tai_jd, 2451545.0);
```
It's important to note that the Gregorian string format must be in the form "YYYY-MM-DDThh:mm:ss:mmm" for the functions to work. 

You can also convert between different Gregorian dates using the `convert_gregorian_to_gregorian` function:
```rust
let utc_greg = convert_gregorian_to_gregorian("2000-01-01T12:00:32.184", Timescale::TT, Timescale::UTC);
assert_eq!(utc_greg, "2000-01-01T11:59:28.000");
let utc_greg = convert_gregorian_to_gregorian("2000-01-01T12:00:32.184", Timescale::TT, Timescale::TDB);
assert_eq!(utc_greg, "2000-01-01T12:00:32.184");
```

These are the *main* conversion functions used within astrorust. So far, they can convert between TAI, UTC, TDB and TT timesystems. 

AstroRust also supports other conversions such as calendar formats. We can convert any string in any timesystem to its calendar equivalent:
```rust
let (year, month, day, hour, minute, second) = convert_from_gregorian_to_calendar(tai_greg);
let expected_val = (2000, 1, 1, 12, 0, 0.0);
assert_eq!((year, month, day, hour, minute, second), expected_val);
```

Likewise, we can take a TAI calendar and format it as a string:
```rust
let tai_cal_to_string = from_tai_calendar_to_gregorian(2000, 1, 1, 0, 0.0, 0.0);
assert_eq!(tai_cal_to_string, "2000-01-01T00:00:00.000");
```
