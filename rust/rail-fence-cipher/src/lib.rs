pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self { rails }
    }

    fn zigzag(&self) -> impl Iterator<Item = u32> {
        (0..self.rails).chain((1..self.rails - 1).rev()).cycle()
    }

    pub fn encode(&self, text: &str) -> String {
        if self.rails <= 1 {
            return text.to_string();
        }

        let mut rails = vec![String::new(); self.rails as usize];

        text.chars()
            .zip(self.zigzag())
            .filter(|(c, _)| c.is_alphanumeric())
            .for_each(|(c, i)| {
                if let Some(s) = rails.get_mut(i as usize) {
                    s.push(c);
                }
            });

        rails.concat()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut indexes: Vec<_> = self.zigzag().zip(0..).take(cipher.len()).collect();
        indexes.sort();

        let mut indexes_char: Vec<_> = cipher
            .chars()
            .zip(indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();
        indexes_char.sort();

        indexes_char.iter().map(|(_, c)| c).collect()
    }
}
