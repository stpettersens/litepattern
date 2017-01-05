pub struct LPattern {
    groups: Vec<String>,
}

impl LPattern {
    pub fn new(pattern: &str) -> LPattern {
        let mut split = pattern.split("%");
        let mut groups: Vec<String> = Vec::new();
        for s in split {
            if s.to_owned().len() > 0 {
                groups.push(s.to_owned())
            }
        }
        println!("{:?}", groups);
        LPattern {
            groups: groups,
        }
    }

    pub fn apply_to(&self, text: &str) {
        println!("Appying to pattern: {:?}", self.groups);
    }
}
