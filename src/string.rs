use core::{
    ops::{Deref, DerefMut},
    slice,
    str::Utf8Error,
};

use crate::{Error, Vec};

pub struct String<const N: usize> {
    vec: Vec<u8, N>,
}

#[allow(dead_code)]
impl<const N: usize> String<N> {
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn from_utf8(v: Vec<u8, N>) -> Result<String<N>, Utf8Error> {
        {
            let ptr = v.data.as_ptr() as *const u8;
            let slice = unsafe { slice::from_raw_parts(ptr, v.len) };
            str::from_utf8(slice)?;
        }

        Ok(String { vec: v })
    }

    pub fn from_utf8_unchecked(v: Vec<u8, N>) -> String<N> {
        String { vec: v }
    }

    pub fn into_bytes(self) -> Vec<u8, N> {
        self.vec
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.vec[..]
    }

    pub fn push(&mut self, c: char) -> Result<(), Error> {
        match c.len_utf8() {
            1 => self.vec.push(c as u8),
            _ => self
                .vec
                .extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), Error> {
        self.vec.extend_from_slice(s.as_bytes())
    }

    pub fn truncate(&mut self, len: usize) {
        self.vec.truncate(len)
    }

    pub fn pop(&mut self) -> Option<u8> {
        let ch = self.chars().rev().next()?;

        for _ in 0..ch.len_utf8() {
            self.vec.pop();
        }

        Some(ch as u8)
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.vec) }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.vec) }
    }
}

impl<const N: usize> core::fmt::Debug for String<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let slice: &str = self;
        slice.fmt(f)
    }
}

impl<const N: usize> PartialEq<&str> for String<N> {
    fn eq(&self, other: &&str) -> bool {
        &self.as_str() == other
    }
}

impl<const N: usize> Deref for String<N> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> DerefMut for String<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl<const N: usize> From<&str> for String<N> {
    fn from(value: &str) -> Self {
        let mut new = String::new();
        new.push_str(value).unwrap();
        new
    }
}

#[cfg(test)]
mod tests {

    use crate::{Vec, string::String};

    #[test]
    fn new() {
        let s: String<3> = String::new();

        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn len() {
        let s: String<6> = "abc".into();

        assert_eq!(s.len(), 3);
    }

    #[test]
    fn capacity() {
        let s: String<6> = "abc".into();

        assert_eq!(s.capacity(), 6);
    }

    #[test]
    fn from() {
        let s: String<3> = "abc".into();

        assert!(!s.is_empty());
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn from_utf8() {
        let mut v: Vec<u8, 3> = Vec::new();
        v.push('a' as u8).unwrap();
        v.push('b' as u8).unwrap();

        let s = String::from_utf8(v).unwrap();
        assert_eq!(s.as_str(), "ab");
    }

    #[test]
    fn from_utf8_unchecked() {
        let mut v: Vec<u8, 3> = Vec::new();
        v.push(0).unwrap();
        v.push(159).unwrap();

        let v = String::from_utf8_unchecked(v);

        assert!(!v.is_empty());
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn into_bytes() {
        let s: String<3> = "abc".into();

        let actual: Vec<u8, 3> = s.into_bytes();

        assert_eq!(actual.len, 3);
        assert_eq!(&actual[..], &['a' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn as_bytes() {
        let s: String<3> = "abc".into();

        let actual: &[u8] = s.as_bytes();

        assert_eq!(&actual[..], &['a' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn as_bytes_mut() {
        let mut s: String<3> = "abc".into();

        let actual: &mut [u8] = s.as_bytes_mut();
        actual[0] = 'z' as u8;

        assert_eq!(&actual[..], &['z' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn push() {
        let mut s: String<4> = String::from("a");

        let alpha = '\u{03B1}'; // bytes length of 2
        assert!(s.push(alpha).is_ok());
        assert!(s.push('b').is_ok());

        assert_eq!(s.as_str().as_bytes(), "aαb".as_bytes());

        assert!(s.push('c').is_err());
    }

    #[test]
    fn push_str() {
        let expected: String<3> = "abc".into();

        let mut s: String<3> = String::new();

        assert!(s.push_str("abc").is_ok());
        assert!(!s.is_empty());
        assert_eq!(s.len(), 3);
        assert_eq!(s, expected.as_str());
    }

    #[test]
    fn truncate() {
        let mut s: String<4> = "abcd".into();

        s.truncate(2);

        assert_eq!(s.len(), 2);
        assert_eq!(s.as_str(), "ab");
    }

    #[test]
    fn pop() {
        let mut s: String<4> = "a".into();

        assert!(s.push('b').is_ok());
        let alpha = '\u{03B1}'; // bytes length of 2
        assert!(s.push(alpha).is_ok());

        assert!(s.pop().is_some());
        assert_eq!(s.as_str().as_bytes(), "ab".as_bytes());
    }

    #[test]
    fn is_empty() {
        let mut s: String<4> = String::new();

        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
        assert!(s.is_empty());

        let _ = s.push('a');
        assert!(!s.is_empty());

        assert!(s.pop().is_some());
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
    }

    #[test]
    fn clear() {
        let mut s: String<4> = "abcd".into();

        s.clear();

        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
    }

    #[test]
    fn as_str() {
        let s: String<3> = "abc".into();

        assert_eq!(s.as_str(), "abc");
    }

    #[test]
    fn as_mut_str() {
        let mut s: String<3> = "abc".into();

        let s = s.as_mut_str();
        s.make_ascii_uppercase();

        assert_eq!(s, "ABC");
    }

    #[test]
    fn deref() {
        let s: String<4> = "abcd".into();
        let str: &str = &s;

        assert_eq!(&str, &"abcd");
    }

    #[test]
    fn deref_mut() {
        let mut s: String<4> = "abcd".into();

        s.make_ascii_uppercase();

        assert_eq!(&s, &"ABCD");
    }
}
