use core::{cmp::Ordering, marker::PhantomData, mem, slice};

use crate::Vec;

pub enum Max {}
pub enum Min {}

pub trait Kind {
    fn ordering() -> Ordering;
}

impl Kind for Max {
    fn ordering() -> Ordering {
        Ordering::Greater
    }
}

impl Kind for Min {
    fn ordering() -> Ordering {
        Ordering::Less
    }
}

pub struct BinaryHeap<T, const N: usize, KIND> {
    _marker: PhantomData<KIND>,
    data: Vec<T, N>,
}

impl<T, const N: usize, K> BinaryHeap<T, N, K>
where
    K: Kind,
    T: Ord,
{
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
            data: Vec::new(),
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn push(&mut self, item: T) -> Result<(), crate::Error> {
        let result = self.data.push(item);

        match result {
            Err(_) => result,
            Ok(_) => {
                self.push_element_up();
                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                mem::swap(&mut item, &mut self.data[0]);
                self.push_elements_down();
            }
            item
        })
    }

    fn push_element_up(&mut self) {
        let mut child_index = self.data.len() - 1;
        while child_index > 0 {
            let parent_index = (child_index - 1) / 2;

            if (self.data[child_index]).cmp(&self.data[parent_index]) == K::ordering() {
                self.data.swap(child_index, parent_index);
                child_index = parent_index;
            } else {
                break;
            }
        }
    }

    fn push_elements_down(&mut self) {
        let mut current = 0;
        let end = self.data.len();

        while current < end {
            let left_child = current * 2 + 1;
            let right_child = current * 2 + 2;

            if left_child >= end {
                break;
            }

            let mut target = current;
            if (self.data[left_child]).cmp(&self.data[target]) == K::ordering() {
                target = left_child;
            }

            if right_child < end
                && (self.data[right_child]).cmp(&self.data[target]) == K::ordering()
            {
                target = right_child;
            }

            if (self.data[target]).cmp(&self.data[current]) == K::ordering() {
                self.data.swap(target, current);
                current = target;
            } else {
                break;
            }
        }
    }
}

impl<'a, T, const N: usize, K> IntoIterator for &'a BinaryHeap<T, N, K>
where
    K: Kind,
    T: Ord,
{
    type Item = &'a T;

    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_heap::{BinaryHeap, Max, Min};

    #[test]
    fn new() {
        let _bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();
    }

    #[test]
    fn capacity() {
        let bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();

        assert_eq!(bh.capacity(), 5);
    }

    #[test]
    fn len() {
        let bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();

        assert_eq!(bh.len(), 0);
    }

    #[test]
    fn is_empty() {
        let bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();

        assert_eq!(bh.is_empty(), true);
    }

    #[test]
    fn iter() {
        let bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();

        assert_eq!(bh.iter().next(), None);
    }

    #[test]
    fn push_max() {
        let mut bh: BinaryHeap<i32, 5, Max> = BinaryHeap::new();

        let _ = bh.push(4);
        let _ = bh.push(2);
        let _ = bh.push(3);
        let _ = bh.push(5);

        let mut iter = bh.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
    }

    #[test]
    fn push_min() {
        let mut bh: BinaryHeap<i32, 5, Min> = BinaryHeap::new();

        let _ = bh.push(4);
        let _ = bh.push(3);
        let _ = bh.push(2);

        let mut iter = bh.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn pop_max() {
        let mut bh: BinaryHeap<i32, 6, Max> = BinaryHeap::new();

        let _ = bh.push(1);
        let _ = bh.push(2);
        let _ = bh.push(7);
        let _ = bh.push(17);
        let _ = bh.push(3);
        let _ = bh.push(10);

        assert_eq!(bh.pop().unwrap(), 17);

        let mut iter = bh.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));

        assert_eq!(bh.pop().unwrap(), 10);
        assert_eq!(bh.pop().unwrap(), 7);
        assert_eq!(bh.pop().unwrap(), 3);
        assert_eq!(bh.pop().unwrap(), 2);
        assert_eq!(bh.pop().unwrap(), 1);
    }

    #[test]
    fn pop_min() {
        let mut bh: BinaryHeap<i32, 6, Min> = BinaryHeap::new();

        let _ = bh.push(7);
        let _ = bh.push(17);
        let _ = bh.push(10);
        let _ = bh.push(1);
        let _ = bh.push(5);
        let _ = bh.push(3);

        assert_eq!(bh.pop().unwrap(), 1);

        let mut iter = bh.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&17));
        assert_eq!(iter.next(), Some(&7));

        assert_eq!(bh.pop().unwrap(), 3);
        assert_eq!(bh.pop().unwrap(), 5);
        assert_eq!(bh.pop().unwrap(), 7);
        assert_eq!(bh.pop().unwrap(), 10);
        assert_eq!(bh.pop().unwrap(), 17);
    }
}
