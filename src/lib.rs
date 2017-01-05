pub struct LPattern {
    groups: Vec<String>,
}

impl LPattern {
    pub fn new(pattern: &str) -> LPattern {
        let mut split = pattern.split("%");
        let mut groups: Vec<String> = Vec::new();
        for s in split {
            groups.push(s.to_owned())
        }
        println!("{:?}", groups);
        LPattern {
            groups: groups,
        }
    }

    pub fn apply_to(&self, text: &str) {

    }
}
