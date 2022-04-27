/// Incremental calculation of mean.
/// See [formula (4)](https://studylib.net/doc/7944244/incremental-calculation-of-weighted-mean-and-variance).
///
/// Incremental calculation of mean produces more numerically stable result than straightforward formula.
/// Detailed information how it happens see [here](https://diego.assencio.com/?index=c34d06f4f4de2375658ed41f70177d59).
/// Incremental calculation also does not suffer from overflow if sum is too large.
///
/// Downside of incremental calculation is that it's slower.
/// Measurements shows about 10% degradation in performance in case of pure calculations without involving IO operations for data read.
/// However IO operations for data read will be the main bottleneck so such performance degradation
/// will become negletable in overall performance.
///
/// As such this way of mean calculation is chosen.
#[derive(Debug, PartialEq)]
pub struct MeanIncremental {
    value: f64,
}

impl MeanIncremental {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn add(&mut self, x: f64, count: u32) {
        self.value = self.value + (x - self.value) / count as f64
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests;
