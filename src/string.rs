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

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len
    }

    pub fn from(s: &str) -> Self {
        let mut new = String::new();
        let _ = new.push_str(s);
        new
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
}

#[cfg(test)]
mod tests {

    use crate::string::String;

    #[test]
    fn new() {
        let s: String<3> = String::new();

        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn from() {
        let s: String<3> = String::from("abc");

        assert!(!s.is_empty());
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn push_str() {
        let mut s: String<4> = String::new();

        assert!(s.push_str("abc").is_ok());
        assert!(!s.is_empty());
        assert_eq!(s.len(), 3);
    }
}
