pub struct LPattern;

impl LPattern {
    pub fn new(pattern: &str) {
        let mut split = pattern.split(")");
        for s in split {
            println!("{}", s);
        }
    }
}
