use core::{marker::PhantomData, mem::MaybeUninit, ops, slice};

#[allow(dead_code)]
pub struct Vec<T, const N: usize> {
    pub(crate) _marker: PhantomData<T>,
    pub(crate) data: MaybeUninit<[T; N]>,
    pub(crate) len: usize,
}

#[allow(dead_code)]
pub enum Error {
    Full,
}

#[allow(dead_code)]
impl<T, const N: usize> Vec<T, N> {
    pub const fn new() -> Self {
        Vec {
            _marker: PhantomData,
            data: MaybeUninit::uninit(),
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn push(&mut self, item: T) -> Result<(), Error> {
        let capacity = self.capacity();

        if self.len < capacity {
            let collection_start_ptr = self.data.as_mut_ptr() as *mut T;
            unsafe { core::ptr::write(collection_start_ptr.add(self.len), item) };
            self.len += 1;

            Ok(())
        } else {
            Err(Error::Full)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let collection_start_ptr = self.data.as_mut_ptr() as *mut T;
            let item_ptr = unsafe { collection_start_ptr.add(self.len) };
            let item = unsafe { core::ptr::read(item_ptr) };

            Some(item)
        }
    }
}

impl<T, const N: usize> Drop for Vec<T, N> {
    fn drop(&mut self) {
        let collection_ptr = self.data.as_mut_ptr() as *mut T;

        let slice = core::ptr::slice_from_raw_parts_mut(collection_ptr, self.len);

        unsafe { core::ptr::drop_in_place(slice) };
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vec<T, N> {
    type Item = &'a T;

    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vec<T, N> {
    type Item = &'a mut T;

    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> ops::Deref for Vec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let collection_ptr = self.data.as_ptr() as *mut T;

        let raw_ptr: *const [T] = core::ptr::slice_from_raw_parts(collection_ptr, self.len);
        let slice: &[T] = unsafe { &*raw_ptr };

        slice
    }
}

impl<T, const N: usize> ops::DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let collection_ptr = self.data.as_mut_ptr() as *mut T;

        let raw_ptr: *mut [T] = core::ptr::slice_from_raw_parts_mut(collection_ptr, self.len);
        unsafe { &mut *raw_ptr }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::{self, Vec};

    #[test]
    fn new() {
        let v: Vec<i8, 0> = vec::Vec::new();

        assert_eq!(v.len, 0);
    }

    #[test]
    fn capacity() {
        let v: Vec<i8, 2> = vec::Vec::new();

        assert_eq!(v.capacity(), 2);
    }

    #[test]
    fn push() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        assert_eq!(v.len, 3);
    }

    #[test]
    fn pop() {
        let mut v: Vec<i8, 5> = vec::Vec::new();
        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        let item = v.pop();

        assert_eq!(v.len, 2);
        assert_eq!(item, Some(3));
    }

    #[test]
    fn drop() {
        struct Droppable;
        impl Droppable {
            fn new() -> Self {
                unsafe {
                    COUNT += 1;
                }
                Droppable
            }
        }
        impl Drop for Droppable {
            fn drop(&mut self) {
                unsafe {
                    COUNT -= 1;
                }
            }
        }

        static mut COUNT: i32 = 0;

        {
            let mut v: Vec<Droppable, 2> = Vec::new();
            let _ = v.push(Droppable::new());
            let _ = v.push(Droppable::new());
        }

        assert_eq!(unsafe { COUNT }, 0);
    }

    #[test]
    fn iter() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        let mut items = v.iter();

        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));

        let _ = v.pop();

        let mut items = v.iter();
        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        let mut items = v.iter_mut();

        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), Some(&mut 3));

        let _ = v.pop();

        let mut items = v.iter_mut();
        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), None);
    }

    }

    #[test]
    fn into_iter() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        let mut items = v.into_iter();

        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));
    }

    #[test]
    fn into_iter_mut() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        for item in &mut v {
            *item *= 10; // Mutate items in-place
        }

        let mut items = v.into_iter();

        assert_eq!(items.next(), Some(&10));
        assert_eq!(items.next(), Some(&20));
        assert_eq!(items.next(), Some(&30));
    }

    #[test]
    fn deref() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        let slice: &[i8] = &v;

        assert_eq!(slice.len(), 3);
    }

    #[test]
    fn deref_mut() {
        let mut v: Vec<i8, 5> = vec::Vec::new();

        let _ = v.push(1);
        let _ = v.push(2);
        let _ = v.push(3);

        v[0] = 10;
        v[1] *= 10;
        v[2] *= 10;

        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
        assert_eq!(v[2], 30);
    }
}
