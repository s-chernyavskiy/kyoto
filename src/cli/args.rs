use std::env::args;

pub struct Args {
    index: usize,
    args: Vec<String>,
}

impl Args {
    pub fn build() -> Self {
        Self::new(args().collect())
    }

    pub fn new(args: Vec<String>) -> Self {
        Self { index: 0, args }
    }

    pub fn get(&self) -> Vec<String> {
        self.args[self.index..].to_vec()
    }

    pub fn curr(&self) -> &String {
        &self.args[self.index]
    }

    pub fn change(&mut self, d: usize) {
        self.index += d;
    }
}
