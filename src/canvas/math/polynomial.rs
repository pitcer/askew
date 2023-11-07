use tinyvec::TinyVec;

#[derive(Debug)]
pub struct Polynomial {
    coefficients: TinyVec<[f32; 4]>,
}

impl Polynomial {
    #[must_use]
    pub fn new(coefficients: TinyVec<[f32; 4]>) -> Self {
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
            .enumerate()
            .skip(1)
            .map(|(index, coefficient)| index as f32 * coefficient)
            .collect();
        Self::new(coefficients)
    }

    #[must_use]
    pub fn find_root(&self, iterations: usize, guess: f32) -> f32 {
        let derivative = self.derivative();
        (0..iterations)
            .fold(guess, |root, _| root - self.evaluate(root) / derivative.evaluate(root))
    }
}

#[cfg(test)]
mod tests {
    use tinyvec::tiny_vec;

    use super::*;

    #[test]
    fn evaluation() {
        let polynomial = Polynomial::new(tiny_vec![1.0, 2.0, 3.0]);
        let actual = polynomial.evaluate(0.5);
        assert!(f32::abs(2.75 - actual) < f32::EPSILON);
        let actual = polynomial.evaluate(0.0);
        assert!(f32::abs(1.0 - actual) < f32::EPSILON);
        let actual = polynomial.evaluate(2.0);
        assert!(f32::abs(17.0 - actual) < f32::EPSILON);
    }

    #[test]
    fn derivative() {
        let polynomial = Polynomial::new(tiny_vec![1.0, 2.0, 3.0]);
        let derivative = polynomial.derivative();
        assert_eq!(tiny_vec![2.0, 6.0], derivative.coefficients);
    }
}
