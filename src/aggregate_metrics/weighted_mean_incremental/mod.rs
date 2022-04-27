/// Incremental calculation of weighted mean.
/// See [formula (53)](https://studylib.net/doc/7944244/incremental-calculation-of-weighted-mean-and-variance).
///
/// See also [super::MeanIncremental] for explanation why incremental calculation is chosen.
#[derive(Debug, PartialEq)]
pub struct WeightedMeanIncremental {
    value: f64,
}

impl WeightedMeanIncremental {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn add(&mut self, x: f64, w: f64, weighted_sum: f64) {
        self.value += (w / weighted_sum) * (x - self.value);
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests;
