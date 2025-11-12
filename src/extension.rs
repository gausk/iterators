use crate::Flatten;

pub trait IteratorExt: Iterator {
    fn flat_it(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator + Sized,
    T::Item: IntoIterator,
{
    fn flat_it(self) -> Flatten<Self> {
        Flatten::new(self)
    }
}

#[test]
fn test_iter_ext() {
    assert_eq!(
        vec![vec![1, 2], vec![3, 4, 5]]
            .into_iter()
            .flat_it()
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
}
