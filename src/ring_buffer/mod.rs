use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

mod spsc;

pub struct RingBuffer<T, const N: usize> {
    _marker: PhantomData<T>,
    buffer: UnsafeCell<MaybeUninit<[T; N]>>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Full,
}

#[allow(dead_code)]
impl<T, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
            buffer: UnsafeCell::new(MaybeUninit::uninit()),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), Error> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);

        if (tail + 1) % N == head {
            return Err(Error::Full);
        }

        let collection_start_ptr = self.buffer.get_mut().as_mut_ptr() as *mut T;
        unsafe {
            core::ptr::write(collection_start_ptr.add(tail), item);
        }
        self.tail.store((tail + 1) % N, Ordering::Release);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);

        if head == tail {
            return None;
        }

        let collection_start_ptr = self.buffer.get_mut().as_mut_ptr() as *mut T;
        let item_ptr = unsafe { collection_start_ptr.add(head) };
        let item = unsafe { core::ptr::read(item_ptr) };

        self.head.store((head + 1) % N, Ordering::Release);
        Some(item)
    }

    fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        head.abs_diff(tail)
    }

    fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            rb: self,
            index: 0,
            len: self.len(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        let len = self.len();
        IterMut {
            rb: self,
            index: 0,
            len,
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a RingBuffer<T, N> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut RingBuffer<T, N> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct Iter<'a, T, const N: usize> {
    rb: &'a RingBuffer<T, N>,
    index: usize,
    len: usize,
}

pub struct IterMut<'a, T, const N: usize> {
    rb: &'a mut RingBuffer<T, N>,
    index: usize,
    len: usize,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let buffer: &MaybeUninit<[T; N]> = unsafe { self.rb.buffer.as_ref_unchecked() };
            let collection_start_ptr: *const T = buffer.as_ptr().cast::<T>();
            let item_ptr = unsafe {
                let head_offset = self.rb.head.load(Ordering::Relaxed);
                collection_start_ptr.add(head_offset + self.index)
            };
            self.index = (self.index + 1) % N;
            Some(unsafe { &*item_ptr })
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let collection_start_ptr: *mut T = self.rb.buffer.get_mut().as_mut_ptr().cast::<T>();
            let item_ptr = unsafe {
                let head_offset = self.rb.head.load(Ordering::Relaxed);
                collection_start_ptr.add(head_offset + self.index)
            };
            self.index = (self.index + 1) % N;
            Some(unsafe { &mut *item_ptr })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::Ordering;

    use crate::ring_buffer::Error;
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn new() {
        let rb: RingBuffer<i8, 5> = RingBuffer::new();

        assert_eq!(rb.head.load(Ordering::Relaxed), 0);
        assert_eq!(rb.tail.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn capacity() {
        let rb: RingBuffer<i8, 5> = RingBuffer::new();

        assert_eq!(rb.capacity(), 5);
    }

    #[test]
    fn enqueue() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        assert_eq!(rb.tail.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn enqueue_full() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        let result = rb.enqueue(3);

        assert_eq!(result, Err(Error::Full));
        assert_eq!(rb.tail.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn dequeue() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        let item = rb.dequeue();

        assert_eq!(item, Some(1));
        assert_eq!(rb.head.load(Ordering::Relaxed), 1);
        assert_eq!(rb.tail.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn dequeue_none() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        let _ = rb.dequeue();
        let _ = rb.dequeue();

        let item = rb.dequeue();

        assert_eq!(item, None);
        assert_eq!(rb.head.load(Ordering::Relaxed), 2);
        assert_eq!(rb.tail.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn len() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        assert_eq!(rb.len(), 0);

        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);
        assert_eq!(rb.len(), 2);

        let _ = rb.dequeue();
        assert_eq!(rb.len(), 1);

        let _ = rb.dequeue();
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn iter() {
        let mut rb: RingBuffer<i8, 4> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);
        let _ = rb.enqueue(3);

        let mut items = rb.iter();

        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut rb: RingBuffer<i8, 4> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);
        let _ = rb.enqueue(3);

        let mut items = rb.iter_mut();

        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), Some(&mut 3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut rb: RingBuffer<i8, 4> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);
        let _ = rb.enqueue(3);

        let mut items = rb.into_iter();

        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));
        assert_eq!(items.next(), None);
    }
}
