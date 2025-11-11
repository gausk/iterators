pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter> where I: IntoIterator, I::Item: IntoIterator {
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O> where O: Iterator, O::Item: IntoIterator {
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O> where O: Iterator, O::Item: IntoIterator {
    pub fn new(outer: O) -> Self {
        Self { outer, inner: None }
    }
}

impl<O> Iterator for Flatten<O> where O: Iterator, O::Item: IntoIterator {
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.inner {
            if let Some(v) = inner.next() {
                return Some(v);
            }
            self.inner = None;
        }
        self.inner = Some(self.outer.next()?.into_iter());
        self.next()
    }
}

#[test]
fn test_multiple() {
    let flat: Vec<_> = flatten(vec![vec![1, 2], vec![3, 4, 5]]).collect();
    assert_eq!(flat, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
}

#[test]
fn test_one() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
}

#[test]
fn test_two() {
    assert_eq!(
        flatten(std::iter::once(vec!["a", "b"])).collect::<Vec<_>>(),
        vec!["a", "b"]
    );
}

#[test]
fn test_two_wide() {
    assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).collect::<Vec<_>>(), vec!["a", "b"]);
}

