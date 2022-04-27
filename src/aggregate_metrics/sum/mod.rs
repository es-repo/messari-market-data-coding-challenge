#[derive(Debug, PartialEq)]
pub struct Sum {
    value: f64,
}

impl Sum {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn add(&mut self, x: f64) {
        self.value += x;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests;
