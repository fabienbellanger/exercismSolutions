use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

/// List of robot names already given
static HISTORY: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    let list = HashSet::new();
    Mutex::new(list)
});

pub struct Robot {
    pub name: String, // (Ex.: FAL239)
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

impl Robot {
    fn generate_name() -> String {
        let mut rng = rand::thread_rng();

        let name = loop {
            if let Ok(mut history) = HISTORY.lock() {
                let robot_name = String::from_utf8_lossy(&[
                    rng.gen_range(65..=90),
                    rng.gen_range(65..=90),
                    rng.gen_range(48..=57),
                    rng.gen_range(48..=57),
                    rng.gen_range(48..=57),
                ])
                .to_string();

                if !history.contains(&robot_name) {
                    history.insert(robot_name.clone());
                    break robot_name;
                }
            }
        };

        name
    }

    pub fn new() -> Self {
        Self {
            name: Self::generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_name();
    }
}
