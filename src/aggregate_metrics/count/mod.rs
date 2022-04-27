#[derive(Debug, PartialEq)]
pub struct Count {
    value: u32,
}

impl Count {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn add(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests;
