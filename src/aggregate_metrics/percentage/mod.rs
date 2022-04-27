#[derive(Debug, PartialEq)]
pub struct Percentage {
    count_of: u32,
}

impl Percentage {
    pub fn new(count_of: u32) -> Self {
        Self { count_of }
    }

    pub fn add(&mut self) {
        self.count_of += 1;
    }

    pub fn get_value(&self, count: u32) -> f64 {
        self.count_of as f64 / count as f64
    }
}

#[cfg(test)]
mod tests;
