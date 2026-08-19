use core::{marker::PhantomData, mem::MaybeUninit, slice};

pub struct RingBuffer<T, const N: usize> {
    _marker: PhantomData<T>,
    data: MaybeUninit<[T; N]>,
    head: usize,
    tail: usize,
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
            data: MaybeUninit::uninit(),
            head: 0,
            tail: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), Error> {
        let nxt_tail = (self.tail + 1) % self.capacity();
        if nxt_tail != self.head {
            let collection_start_ptr = self.data.as_mut_ptr() as *mut T;
            unsafe {
                core::ptr::write(collection_start_ptr.add(self.tail), item);
            }
            self.tail = nxt_tail;
            Ok(())
        } else {
            Err(Error::Full)
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.head != self.tail {
            let collection_start_ptr = self.data.as_mut_ptr() as *mut T;
            let item_ptr = unsafe { collection_start_ptr.add(self.head) };
            let item = unsafe { core::ptr::read(item_ptr) };

            self.head = (self.head + 1) % self.capacity();
            Some(item)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        if self.head == self.tail {
            0
        } else if self.head > self.tail {
            self.head - self.tail
        } else {
            self.tail - self.head
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::ring_buffer::Error;
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn new() {
        let rb: RingBuffer<i8, 5> = RingBuffer::new();

        assert_eq!(rb.head, 0);
        assert_eq!(rb.tail, 0);
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

        assert_eq!(rb.tail, 2);
    }

    #[test]
    fn enqueue_full() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        let result = rb.enqueue(3);

        assert_eq!(result, Err(Error::Full));
        assert_eq!(rb.tail, 2);
    }

    #[test]
    fn dequeue() {
        let mut rb: RingBuffer<i8, 3> = RingBuffer::new();
        let _ = rb.enqueue(1);
        let _ = rb.enqueue(2);

        let item = rb.dequeue();

        assert_eq!(item, Some(1));
        assert_eq!(rb.head, 1);
        assert_eq!(rb.tail, 2)
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
        assert_eq!(rb.head, 2);
        assert_eq!(rb.tail, 2)
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

}
