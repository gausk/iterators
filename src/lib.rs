pub mod extension;
pub mod flat_map;
mod iter;

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_back: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(outer: O) -> Self {
        Self {
            outer,
            inner: None,
            inner_back: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.inner {
            if let Some(v) = inner.next() {
                return Some(v);
            }
            self.inner = None;
        }
        match self.outer.next() {
            Some(v) => {
                self.inner = Some(v.into_iter());
            }
            None => {
                return self.inner_back.as_mut()?.next();
            }
        }
        self.next()
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.inner_back {
            if let Some(v) = inner.next_back() {
                return Some(v);
            }
            self.inner_back = None;
        }
        match self.outer.next_back() {
            Some(v) => {
                self.inner_back = Some(v.into_iter());
            }
            None => return self.inner.as_mut()?.next_back(),
        }
        self.next_back()
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
    assert_eq!(
        flatten(vec![vec!["a"], vec!["b"]]).collect::<Vec<_>>(),
        vec!["a", "b"]
    );
}

#[test]
fn test_double_ended_reverse() {
    assert_eq!(
        flatten(vec![vec![1, 2], vec![3, 4, 5]])
            .rev()
            .collect::<Vec<_>>(),
        vec![5, 4, 3, 2, 1]
    );
}

#[test]
fn test_double_ended_back() {
    let mut flat = flatten(vec![vec![1, 2], vec![3]]);
    assert_eq!(flat.next(), Some(1));
    assert_eq!(flat.next_back(), Some(3));
    assert_eq!(flat.next_back(), Some(2));
    assert_eq!(flat.next(), None);
    assert_eq!(flat.next_back(), None);
}

#[test]
fn test_double_ended() {
    let mut flat = flatten(vec![vec![1], vec![2, 3]]);
    assert_eq!(flat.next(), Some(1));
    assert_eq!(flat.next_back(), Some(3));
    assert_eq!(flat.next(), Some(2));
    assert_eq!(flat.next(), None);
    assert_eq!(flat.next_back(), None);
}

#[test]
fn test_inf() {
    let mut flat = flatten((0..).map(|i| 0..i));
    assert_eq!(flat.next(), Some(0));
    assert_eq!(flat.next(), Some(0));
    assert_eq!(flat.next(), Some(1));
    assert_eq!(flat.next(), Some(0));
}

#[test]
fn test_multiple_level_deep() {
    assert_eq!(flatten(vec![vec![vec![1, 2]]]).count(), 1);
    assert_eq!(flatten(flatten(vec![vec![vec![1, 2]]])).count(), 2);
}
