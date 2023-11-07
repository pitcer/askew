#[derive(Debug)]
pub struct Polynomial {
    coefficients: Vec<f32>,
}

impl Polynomial {
    #[must_use]
    pub fn new(coefficients: Vec<f32>) -> Self {
        debug_assert!(!coefficients.is_empty());
        Self { coefficients }
    }

    #[must_use]
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Assumes that the given polynomial has at least one coefficient.
    #[must_use]
    pub fn evaluate(&self, argument: f32) -> f32 {
        self.coefficients
            .iter()
            .rev()
            .copied()
            .reduce(|value, coefficient| coefficient + argument * value)
            .expect("Polynomial should have at least one coefficient")
    }

    #[must_use]
    pub fn derivative(&self) -> Polynomial {
        let coefficients = self
            .coefficients
            .iter()
            .copied()
            .skip(1)
            .enumerate()
            .map(|(index, coefficient)| index as f32 * coefficient)
            .collect();
        Self::new(coefficients)
    }

    #[must_use]
    pub fn find_root(&self, guess: f32) -> f32 {
        let derivative = self.derivative();
        (0..10).fold(guess, |root, _| root - self.evaluate(root) / derivative.evaluate(root))
    }
}
