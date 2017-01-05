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

    pub fn apply_to(&self, text: &str) -> Vec<String> {
        println!("Applying to pattern: {:?}", self.groups);
        let mut captures: Vec<String> = Vec::new();
        for g in &self.groups {
            let mut substring: Vec<String> = Vec::new();
            for i in 0..g.len() - 1 {
                substring.push(text.chars()[i]);
            }
            captures.push(substring.join(""))
        }
        captures
    }
}
