use std::{collections::HashSet, fmt::Debug, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Hash + PartialEq + Eq> {
    values: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: Hash + PartialEq + Eq + Clone + Debug,
{
    pub fn new(input: &[T]) -> Self {
        let values = input.into_iter().cloned().collect();
        Self { values }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.values.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.values.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        // self.values.iter().all(|v| other.values.contains(v))
        self.values.is_subset(&other.values)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        // !self.values.iter().any(|v| other.values.contains(v))
        self.values.is_disjoint(&other.values)
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
