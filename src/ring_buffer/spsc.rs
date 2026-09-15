use core::{marker::PhantomData, mem::MaybeUninit, sync::atomic::Ordering};

use crate::{RingBuffer, ring_buffer::Error};

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn split(&self) -> (Producer<'_, T, N>, Consumer<'_, T, N>) {
        (
            Producer {
                rb: self,
                _marker: PhantomData,
            },
            Consumer {
                rb: self,
                _marker: PhantomData,
            },
        )
    }
}

pub struct Producer<'a, T, const N: usize> {
    rb: &'a RingBuffer<T, N>,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Producer<'a, T, N> {
    pub fn enqueue(&self, item: T) -> Result<(), Error> {
        let head = self.rb.head.load(Ordering::Acquire);
        let tail = self.rb.tail.load(Ordering::Relaxed);

        if (tail + 1) % N == head {
            return Err(Error::Full);
        }

        unsafe {
            let buffer_raw_ptr = self.rb.buffer.get() as *mut MaybeUninit<T>;
            let item_ptr = buffer_raw_ptr.add(tail) as *mut T;

            core::ptr::write(item_ptr, item);
        }

        self.rb.tail.store((tail + 1) % N, Ordering::Release);
        Ok(())
    }
}

unsafe impl<'a, T, const N: usize> Send for Producer<'a, T, N> {}

pub struct Consumer<'a, T, const N: usize> {
    rb: &'a RingBuffer<T, N>,
    _marker: PhantomData<T>,
}

unsafe impl<'a, T, const N: usize> Send for Consumer<'a, T, N> {}

impl<'a, T, const N: usize> Consumer<'a, T, N> {
    pub fn dequeue(&mut self) -> Option<T> {
        let tail = self.rb.tail.load(Ordering::Acquire);
        let head = self.rb.head.load(Ordering::Relaxed);

        if head == tail {
            return None;
        }

        unsafe {
            let buffer_raw_ptr = self.rb.buffer.get() as *mut MaybeUninit<T>;
            let item_ptr = buffer_raw_ptr.add(head) as *mut T;

            let item = core::ptr::read(item_ptr);
            self.rb.head.store((head + 1) % N, Ordering::Release);
            Some(item)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RingBuffer;

    #[test]
    fn sanity() {
        let rb: RingBuffer<i32, 6> = RingBuffer::new();

        let (producer, mut consumer) = rb.split();

        assert_eq!(consumer.dequeue(), None);
        assert_eq!(producer.enqueue(1), Ok(()));
        assert_eq!(consumer.dequeue(), Some(1));
    }
}
