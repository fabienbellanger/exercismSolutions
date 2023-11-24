use std::fmt;
use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T: Clone> {
    matcher: fn(T) -> bool,
    subs: String,
}
impl<T: Clone> Matcher<T> {
    pub fn new<S: ToString>(matcher: fn(T) -> bool, subs: S) -> Matcher<T> {
        Self {
            matcher,
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T: fmt::Display + Clone> {
    matchers: Vec<Matcher<T>>,
}

impl<T: fmt::Display + Clone> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(matcher);

        Self { matchers }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |e| {
            let res: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(e.clone()))
                .map(|m| m.subs.clone())
                .collect();

            if res.is_empty() {
                e.to_string()
            } else {
                res
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + ToString + PartialEq + From<u8> + fmt::Display + Clone,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
