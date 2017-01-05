pub struct LPattern;

impl LPattern {
    pub fn new(pattern: &str) {
        let mut split = pattern.split(")");
        let groups: Vec<&str> = split.collect();
        println!("{:?}", groups);
    }
}
