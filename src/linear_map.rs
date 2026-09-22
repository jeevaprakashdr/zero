use core::{
    borrow::Borrow,
    mem,
    ops::{Index, IndexMut},
    slice,
};

use crate::{Error, Vec};

struct LinearMap<Key, Value, const N: usize>
where
    Key: Eq,
{
    buffer: Vec<(Key, Value), N>,
}

impl<Key, Value, const N: usize> LinearMap<Key, Value, N>
where
    Key: Eq,
{
    pub const fn new() -> Self {
        LinearMap { buffer: Vec::new() }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn insert(&mut self, key: Key, mut value: Value) -> Result<Option<Value>, Error> {
        if let Some((_, v)) = self.iter_mut().find(|&(k, _)| *k == key) {
            mem::swap(v, &mut value);
            return Ok(Some(value));
        }

        self.buffer.push((key, value))?;
        Ok(None)
    }

    pub fn iter(&self) -> Iter<'_, Key, Value> {
        Iter {
            iter: self.buffer.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Key, Value> {
        IterMut {
            iter: self.buffer.iter_mut(),
        }
    }

    pub fn conatains_key<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.iter()
            .find(|&(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Key: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.iter_mut()
            .find(|&(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: Key) -> Option<Value> {
        let idx = self
            .keys()
            .enumerate()
            .find(|&(_, k)| *k == key)
            .map(|(idx, _)| idx);

        idx.map(|idx| self.buffer.swap_remove(idx).1)
    }
}

impl<'a, K, V, const N: usize, Q> IndexMut<&'a Q> for LinearMap<K, V, N>
where
    K: Borrow<Q> + Eq,
    Q: Eq + ?Sized,
{
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("no entry found for key")
    }
}

impl<'a, K, V, const N: usize, Q> Index<&'a Q> for LinearMap<K, V, N>
where
    K: Borrow<Q> + Eq,
    Q: Eq + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&(ref k, ref v)| (k, v))
    }
}

pub struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&mut (ref k, ref mut v)| (k, v))
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_map::LinearMap;

    #[test]
    fn new() {
        let _: LinearMap<&str, i32, 5> = LinearMap::new();
    }

    #[test]
    fn capacity() {
        let map: LinearMap<&str, i32, 5> = LinearMap::new();

        assert_eq!(map.capacity(), 5);
    }

    #[test]
    fn len() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();

        let _ = map.insert("k1", 11);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();

        assert_eq!(map.is_empty(), true);

        let _ = map.insert("k1", 11);
        assert_eq!(map.is_empty(), false);
    }

    #[test]
    fn clear() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 11);

        map.clear();

        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn insert() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();

        let x = map.insert("k1", 1);

        assert_eq!(x, Ok(None));
        assert_eq!(map.len(), 1);

        let x = map.insert("k1", 2);

        assert_eq!(x, Ok(Some(1)));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn contains_key() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);

        assert_eq!(map.conatains_key("k1"), true);
        assert_eq!(map.conatains_key("k2"), false);
    }

    #[test]
    fn get() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();

        let x = map.insert("k1", 10);

        assert_eq!(x, Ok(None));
        assert_eq!(map.len(), 1);

        assert_eq!(map.get("k1"), Some(&10));
    }

    #[test]
    fn get_mut() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();

        let x = map.insert("k1", 10);

        assert_eq!(x, Ok(None));
        assert_eq!(map.len(), 1);

        let val = map.get_mut("k1").unwrap();
        *val += 20;

        assert_eq!(val, &mut 30);
        assert_eq!(map.get_mut("k1"), Some(&mut 30));
    }

    #[test]
    fn keys() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        let mut iter = map.keys();

        assert_eq!(iter.next(), Some(&"k1"));
        assert_eq!(iter.next(), Some(&"k2"));
        assert_eq!(iter.next(), Some(&"k3"));
        assert_eq!(iter.next(), None)
    }

    #[test]
    fn values() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        let mut iter = map.values();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None)
    }

    #[test]
    fn values_mut() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        let iter = map.values_mut();
        for ele in iter {
            *ele *= 10;
        }

        let mut iter = map.values_mut();
        assert_eq!(iter.next(), Some(&mut 10));
        assert_eq!(iter.next(), Some(&mut 20));
        assert_eq!(iter.next(), Some(&mut 30));
        assert_eq!(iter.next(), None)
    }

    #[test]
    fn remove() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        map.remove("k2");

        let mut iter = map.keys();
        assert_eq!(iter.next(), Some(&"k1"));
        assert_eq!(iter.next(), Some(&"k3"));
        assert_eq!(iter.next(), None)
    }

    #[test]
    fn index() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        assert_eq!(map["k1"], 1);
    }

    #[test]
    fn index_mut() {
        let mut map: LinearMap<&str, i32, 5> = LinearMap::new();
        let _ = map.insert("k1", 1);
        let _ = map.insert("k2", 2);
        let _ = map.insert("k3", 3);

        map["k1"] *= 10;

        assert_eq!(map["k1"], 10);
    }
}
