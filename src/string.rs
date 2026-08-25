use core::{slice, str::Utf8Error};

use crate::Vec;

pub struct String<const N: usize> {
    vec: Vec<u8, N>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Full,
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
        self.vec.len
    }

    pub fn from(s: &str) -> Self {
        let mut new = String::new();
        new.push_str(s).unwrap();
        new
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

    pub fn push(&mut self, c: char) -> Result<(), Error> {
        if self.vec.len + 1 <= self.vec.capacity() {
            let _ = self.vec.push(c as u8);
            Ok(())
        } else {
            Err(Error::Full)
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.bytes();
        if self.vec.len + bytes.len() <= self.vec.capacity() {
            let dst_ptr = self.vec.data.as_mut_ptr() as *mut u8;
            let src_ptr = s.as_ptr() as *mut u8;
            unsafe {
                core::ptr::copy_nonoverlapping(src_ptr, dst_ptr.add(self.vec.len), bytes.len())
            };

            self.vec.len += bytes.len();
            Ok(())
        } else {
            Err(Error::Full)
        }
    }

    pub fn truncate(&mut self, len: usize) {
        self.vec.truncate(len)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.vec.pop()
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn as_str(&self) -> &str {
        let ptr = self.vec.data.as_ptr() as *const u8;
        let slice = unsafe { slice::from_raw_parts(ptr, self.vec.len) };
        unsafe { core::str::from_utf8_unchecked(slice) }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        let ptr = self.vec.data.as_mut_ptr() as *mut u8;
        let slice = unsafe { slice::from_raw_parts_mut(ptr, self.vec.len) };
        unsafe { core::str::from_utf8_unchecked_mut(slice) }
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
        let s: String<6> = String::from("abc");

        assert_eq!(s.len(), 3);
    }

    #[test]
    fn capacity() {
        let s: String<6> = String::from("abc");

        assert_eq!(s.capacity(), 6);
    }

    #[test]
    fn from() {
        let s: String<3> = String::from("abc");

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
        let str = "abc";
        let s: String<3> = String::from(str);

        let actual: Vec<u8, 3> = s.into_bytes();

        assert_eq!(actual.len, 3);
        assert_eq!(&actual[..], &['a' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn as_bytes() {
        let str = "abc";
        let s: String<3> = String::from(str);

        let actual: &[u8] = s.as_bytes();

        assert_eq!(&actual[..], &['a' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn push() {
        let mut s: String<4> = String::from("abc");

        assert!(s.push('1').is_ok());
        assert_eq!(s.as_str(), "abc1");
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn push_str() {
        let mut s: String<4> = String::new();

        assert!(s.push_str("abc").is_ok());
        assert!(!s.is_empty());
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn truncate() {
        let mut s: String<4> = String::from("abcd");

        s.truncate(2);

        assert_eq!(s.len(), 2);
        assert_eq!(s.as_str(), "ab");
    }

    #[test]
    fn pop() {
        let mut s: String<4> = String::from("abcd");

        assert!(s.pop().is_some());
        assert_eq!(s.len(), 3);
        assert_eq!(s.as_str(), "abc");
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
        let mut s: String<4> = String::from("abcd");

        s.clear();

        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
    }

    #[test]
    fn as_str() {
        let s: String<3> = String::from("abc");

        assert_eq!(s.as_str(), "abc");
    }

    #[test]
    fn as_mut_str() {
        let mut s: String<3> = String::from("abc");

        let s = s.as_mut_str();
        s.make_ascii_uppercase();

        assert_eq!(s, "ABC");
    }
}
