pub fn flatmap<I, U, F>(iter: I, f: F) -> FlatMap<I, U, F>
where
    I: Iterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    FlatMap {
        iter,
        f,
        inner: None,
    }
}

pub struct FlatMap<O, U, F>
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    iter: O,
    f: F,
    inner: Option<U::IntoIter>,
}

impl<O, U, F> FlatMap<O, U, F>
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    pub fn new(iter: O, f: F) -> Self {
        Self {
            iter,
            f,
            inner: None,
        }
    }
}

impl<O, U, F> Iterator for FlatMap<O, U, F>
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.inner.as_mut() {
            if let Some(v) = inner.next() {
                return Some(v);
            }
            self.inner = None;
        }
        self.inner = Some((self.f)(self.iter.next()?).into_iter());
        self.next()
    }
}

#[test]
fn test_flatmap() {
    let words = ["alpha", "beta", "gamma"];
    let flatmap = FlatMap::new(words.into_iter(), |s: &str| s.chars());
    assert_eq!(flatmap.collect::<String>(), "alphabetagamma");
}
