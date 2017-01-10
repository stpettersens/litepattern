### litepattern
> Lightweight pattern matching library for Rust.

[![Build Status](https://travis-ci.org/stpettersens/litepattern.png?branch=master)](https://travis-ci.org/stpettersens/litepattern)
[![Build status](https://ci.appveyor.com/api/projects/status/e4prcyy8q1grrdgn?svg=true)](https://ci.appveyor.com/project/stpettersens/litepattern

```toml
# Add to your Cargo.toml file dependencies:
litepattern = "0.1.0" 
# or: litepattern = { git = "https://github.com/stpettersens/litepattern.git" }
```

You can use litepattern as a lighter alternative to the [regex](https://github.com/alexchricton/regex] crate, if you only need to do simple pattern matching. For example, say you want to pass a simple timestamp such as 2017-01-10T19:10:00 and break it down into is constituent parts:

```rust
extern crate litepattern;
use litepattern::LPattern;

fn main() {
  // Parse something like 2017-01-10T19:10:00.
  // The % is mandatory, but the d is just notation for a digit, you can use another character:
  let p = LPattern::new("%dddd-%dd-%ddT%dd-%dd-%dd");
}
```
