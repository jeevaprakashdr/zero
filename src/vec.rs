use core::{marker::PhantomData, mem::MaybeUninit};

#[allow(dead_code)]
pub struct Vec<T, const N: usize> {
    _marker: PhantomData<T>,
    data: MaybeUninit<[T; N]>,
    len: usize,
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

    fn push(&mut self, item: T) -> Result<(), Error> {
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

    fn pop(&mut self) -> Option<T> {
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
}
