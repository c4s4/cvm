use rand::Rng;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CVM {
    size: usize,
    p: f64,
    data: HashSet<String>,
}

impl CVM {
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            p: 1.0,
            data: HashSet::new(),
        }
    }

    pub fn add(&mut self, word: String) {
        self.data.remove(&word);
        if rand::thread_rng().gen_range(0.0..1.0) < self.p {
            self.data.insert(word);
        };
        if self.data.len() >= self.size {
            self.trim();
        };
    }

    pub fn count(&self) -> usize {
        (self.data.len() as f64 / self.p) as usize
    }

    fn trim(&mut self) {
        let mut new_data = HashSet::new();
        self.p /= 2.0;
        for value in &self.data {
            if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
                new_data.insert(value.to_string());
            }
        }
        self.data = new_data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cvm = CVM::new(100);
        cvm.add("hello".to_string());
        cvm.add("world".to_string());
        cvm.add("hello".to_string());
        assert!(cvm.count() == 2);
    }
}
