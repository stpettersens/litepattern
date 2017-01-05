pub struct LPattern {
    pattern: String,
    groups: Vec<String>,
}

impl LPattern {
    pub fn new(pattern: &str) -> LPattern {
        let split = pattern.split("%");
        let mut groups: Vec<String> = Vec::new();
        for s in split {
            if s.to_owned().len() > 0 {
                groups.push(s.to_owned())
            }
        }
        println!("{:?}", groups);
        LPattern {
            pattern: pattern.to_owned(),
            groups: groups,
        }
    }
    
    pub fn apply_to(&self, text: &str) -> Vec<String> {
        let mut i = 0;
        let mut captures: Vec<String> = Vec::new();
        let mut fcaptures: Vec<String> = Vec::new();
        for g in &self.groups {
            let mut substring: Vec<String> = Vec::new();
            for _ in 0..g.len() {
                substring.push(format!("{}", text.chars().nth(i).unwrap()));
                i += 1;
            }
            captures.push(substring.join(""));
        }
        captures
    }
}
