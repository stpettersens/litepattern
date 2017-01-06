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
        LPattern {
            groups: groups,
        }
    }
    
    pub fn apply_to(&self, text: &str) -> Vec<String> {
        let mut i = 0;
        let mut captures: Vec<String> = Vec::new();
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
    
    pub fn is_match(captures: Vec<String>, text: &str) -> bool {
        let mut matched = true;
        let capturesj = captures.join("");
        if text.len() == capturesj.len() {
            for (i, c) in text.chars().enumerate() {
                if c != capturesj.chars().nth(i).unwrap() {
                    matched = false;
                    break;
                }
            }
        } else {
            matched = false;
        }
        matched
    }
}
