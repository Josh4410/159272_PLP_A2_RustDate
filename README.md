# Judgment Date (Rust)

A Gregorian calendar `Date` type in Rust, written for 159.272 Programming Language Paradigms at Massey University (Assignment 2). It replaces an earlier Assignment 1 implementation in another language.

Dates are stored as a single `i32`: days since 0/1/1 (astronomical year 0, January 1st). Negative values are dates before that.

## What it does

- `from_ymd(year, month, day)` and `ymd()` convert between year/month/day and the day count, both forwards and backwards in time
- Leap years follow Gregorian rules (every 4 years, except centuries, except every 400)
- `Display` uses Gregorian BC notation. There is no year 0, so astronomical year 0 prints as `1 BC` and -1 as `2 BC`
- `Add<i32>` gives date arithmetic: `date + 30` is 30 days later
- No external crates

## Run

```sh
cargo run    # prints the expected sequence across 1 BC / AD 1
cargo test   # round trip, leap year and BC display tests
```

```
Date { days: 738885 }
2/12/31 BC
1/1/30 BC
...
1/12/25 BC
1/1/24
...
```

## Result

Graded Excellent: all tests passed, with correct date calculations, leap year handling and BC/AD conversion.

After grading, this version was tidied up: names changed to Rust's snake_case convention (the only feedback point), `Debug` output aligned to the spec, and unit tests added.
