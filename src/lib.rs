pub struct LPattern {
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
            groups: groups,
        }
    }

    pub fn apply_to(&self, text: &str) -> Vec<String> {
        println!("Applying to pattern: {:?}", self.groups);
        let mut chars: Vec<char> = Vec::new();
        for c in text.chars() {
            chars.push(c);
        }
        let mut captures: Vec<String> = Vec::new();
        let mut x = 0;
        for g in &self.groups {
            let mut substring: Vec<String> = Vec::new();
            for i in 0..g.len() {
                x = x + i;
                substring.push(format!("{}", chars[x]));
            }
            captures.push(substring.join(""))
        }
        captures
    }
}
