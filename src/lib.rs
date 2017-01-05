pub struct LPattern {
    groups: Vec<&str>
};

impl LPattern {
    pub fn new(pattern: &str) -> LPattern {
        let mut split = pattern.split("%");
        let groups: Vec<&str> = split.collect();
        println!("{:?}", groups);
        LPattern {
            groups: groups,
        }
    }

    pub fn apply_to(&self, text: &str) {

    }
}
