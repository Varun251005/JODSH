use std::collections::HashMap;

pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: String) {
        self.aliases.insert(name, value);
    }

    pub fn remove(&mut self, name: &str) {
        self.aliases.remove(name);
    }

    pub fn expand(&self, input: &str) -> String {
        // Expand the first word of the input string
        let mut parts = input.splitn(2, ' ');
        if let Some(first_word) = parts.next() {
            if let Some(expanded) = self.aliases.get(first_word) {
                if let Some(rest) = parts.next() {
                    return format!("{} {}", expanded, rest);
                } else {
                    return expanded.clone();
                }
            }
        }
        input.to_string()
    }
}
