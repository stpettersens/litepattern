### litepattern
> Lightweight pattern matching library for Rust.

[![Build Status](https://travis-ci.org/stpettersens/litepattern.png?branch=master)](https://travis-ci.org/stpettersens/litepattern)
[![Build status](https://ci.appveyor.com/api/projects/status/e4prcyy8q1grrdgn?svg=true)](https://ci.appveyor.com/project/stpettersens/litepattern)

```toml
# Add to your Cargo.toml file dependencies:
litepattern = "0.1.0" 
# or: litepattern = { git = "https://github.com/stpettersens/litepattern.git" }
```

You can use `litepattern` as a lighter alternative to the [regex](https://github.com/rust-lang/regex) crate, if you only need to do simple pattern matching. For example, say you want to pass a simple timestamp such as `2017-01-10T19:10:00` and break it down into is constituent parts:

```rust
extern crate litepattern;
use litepattern::LPattern;

fn main() {
  // Parse something like 2017-01-10T19:10:00.
  // The % is mandatory, but the d is just notation for a digit, you can use another non-"%" character.
  let p = LPattern::new("%dddd-%dd-%ddT%dd-%dd-%dd"); // => LPattern.
  
  // Apply the pattern against ("to") some input text and return any matches (captures) as a vector of Strings.
  let caps = p.apply_to("2017-01-10T19:10:00"); // => ["2017-", "01-", "10-", "T19:", "10:", "00"]
  
  // Get the year.
  println!("{}", caps[0][0..4]); // First item in vector; slice of four characters from index zero => 2017
  
  // Get the month.
  println!("{}", caps[1][0..2]); // Second item in vector; slice of two characters from index zero => 01
  
  // Get the day.
  println!("{}", capts[2][0..2]); // Third item in vector; slice of two characters from index zero => 10
}
```

[Documentation](https://docs.rs/litepattern)
