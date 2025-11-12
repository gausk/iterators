use std::marker::PhantomData;

pub struct MyVec<T>(Vec<T>);

impl<'a, T> MyVec<T> {
    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        self.into_iter()
    }
}

pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let ptr = &mut *self.ptr;
                self.ptr = self.ptr.add(1);
                Some(ptr)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a mut MyVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let ptr = self.0.as_mut_ptr();
        let end = unsafe { self.0.as_mut_ptr().add(self.0.len()) };
        IterMut { ptr, end, phantom: PhantomData }
    }
}

#[test]
fn test_iter_mut() {
    let mut my_vec = MyVec(vec![1, 2, 3]);
    let iter_mut = my_vec.iter_mut();
    for elem in iter_mut {
        *elem += 1;
    }
    assert_eq!(my_vec.0, vec![2, 3, 4]);
}

impl<T> IntoIterator for MyVec<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = MyIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        MyIntoIter {
            inner: self.0.clone(),
            index: 0,
        }
    }
}

pub struct MyIntoIter<T>
where
    T: Clone,
{
    inner: Vec<T>,
    index: usize,
}

impl<T> Iterator for MyIntoIter<T>
where
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len() {
            None
        } else {
            self.index += 1;
            Some(self.inner[self.index - 1].clone())
        }
    }
}

#[test]
fn test_my_into_iter() {
    let my_vec = MyVec(vec![1, 2, 3]);
    let mut iter = my_vec.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}
