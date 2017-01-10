/*
    litepattern.
    Light pattern matching library for Rust.

    Copyright (c) 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

//! Light pattern matching library for Rust.

pub struct LPattern {
    length: usize,
    groups: Vec<String>,
}

impl LPattern {

    /// 
    /// Create a new pattern.
    /// E.g. %dddd-%dd-%dd
    ///
    /// * `pattern` - Pattern to create.
    ///
    /// # Return value:
    /// An LPattern.
    pub fn new(pattern: &str) -> LPattern {
        let split = pattern.split("%");
        let mut groups: Vec<String> = Vec::new();
        for s in split {
            if s.to_owned().len() > 0 {
                groups.push(s.to_owned())
            }
        }
        LPattern {
            length: groups.join("").len(),
            groups: groups,
        }
    }
    
    ///
    /// Apply the created pattern to some text.
    /// E.g. 2010-10-10 
    ///
    /// * `text` - Text to apply pattern to.
    ///
    /// # Return value:
    /// Vector of pattern matched strings.
    pub fn apply_to(&self, text: &str) -> Vec<String> {
        let mut i = 0;
        let mut captures: Vec<String> = Vec::new();
        for g in &self.groups {
            let mut substring: Vec<String> = Vec::new();
            for _ in 0..g.len() {
                if i < text.len() {
                    substring.push(format!("{}", text.chars().nth(i).unwrap()));
                }
                i += 1;
            }
            captures.push(substring.join(""));
        }
        captures
    }
    
    ///
    /// Determine if the pattern matches from the capture results and text.
    ///
    /// * `captures` - Captures returned from apply_to method.
    /// * `text` - Text to apply pattern to.
    ///
    /// # Return value:
    /// Did the pattern match against capture results and text?
    pub fn is_match(&self, captures: Vec<String>, text: &str) -> bool {
        let mut matched = false;
        let capturesj = captures.join("");
        if text.len() == self.length {
            matched = true;
            for (i, c) in text.chars().enumerate() {
                if c != capturesj.chars().nth(i).unwrap() {
                    matched = false;
                    break;
                }
            }
        }
        matched
    }
}

#[cfg(test)]
#[test]
fn test_matching_pattern() {
    let t = "2010-10-10";
    let p = LPattern::new("%dddd-%dd-%dd");
    let caps = p.apply_to(&t);
    assert_eq!(p.is_match(caps, &t), true);
}

#[test]
fn test_failing_pattern() {
    let t = "spaghetti";
    let p = LPattern::new("%dddd-%dd-%dd");
    let caps = p.apply_to(&t);
    assert_eq!(p.is_match(caps, &t), false);
}
