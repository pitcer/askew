use bitvec::prelude::BitVec;

#[derive(Debug)]
pub struct IdAssigner(BitVec);

impl IdAssigner {
    #[must_use]
    pub fn new() -> Self {
        Self(BitVec::new())
    }

    pub fn from_assigned_ids(ids: impl Iterator<Item = usize>) -> Self {
        let mut bits = BitVec::new();
        for id in ids {
            if bits.len() < id + 1 {
                bits.resize(id + 1, false);
            }
            bits.set(id, true);
        }
        Self(bits)
    }

    /// Finds and returns the lowest non-assigned id.
    pub fn assign_id(&mut self) -> usize {
        let free_id = self.0.leading_ones();
        if free_id == self.0.len() {
            self.0.push(true);
        } else {
            let result = self.0.replace(free_id, true);
            debug_assert!(!result, "id {free_id} is already assigned");
        }
        free_id
    }

    /// Assumes that the given id is assigned and marks it as not assigned.
    pub fn remove_id(&mut self, id: usize) {
        let result = self.0.replace(id, false);
        debug_assert!(result, "id {id} is not assigned");
        self.truncate();
    }

    fn truncate(&mut self) {
        let trailing_zeros = self.0.trailing_zeros();
        let length = self.0.len();
        self.0.truncate(length - trailing_zeros);
    }
}

impl Default for IdAssigner {
    fn default() -> Self {
        Self::new()
    }
}
