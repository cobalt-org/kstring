use std::fmt;

#[allow(unused)]
const TAG_SIZE: usize = std::mem::size_of::<u8>();

type Len = u8;
#[allow(unused)]
const LEN_SIZE: usize = std::mem::size_of::<Len>();

#[allow(unused)]
const MAX_CAPACITY: usize = std::mem::size_of::<crate::string::StdString>() - TAG_SIZE - LEN_SIZE;

// Performance seems to slow down when trying to occupy all of the padding left by `String`'s
// discriminant.  The question is whether faster len=1-16 "allocations" outweighs going to the heap
// for len=17-22.
#[allow(unused)]
const ALIGNED_CAPACITY: usize = std::mem::size_of::<crate::string::OwnedStr>() - LEN_SIZE;

#[cfg(feature = "max_inline")]
pub(crate) const CAPACITY: usize = MAX_CAPACITY;
#[cfg(not(feature = "max_inline"))]
pub(crate) const CAPACITY: usize = ALIGNED_CAPACITY;

#[derive(Copy, Clone)]
pub struct StackString<const C: usize> {
    len: Len,
    buffer: StrBuffer<C>,
}

impl<const C: usize> StackString<C> {
    pub const CAPACITY: usize = C;
    pub const EMPTY: Self = Self::empty();

    const fn empty() -> Self {
        Self {
            len: 0,
            buffer: StrBuffer::empty(),
        }
    }

    #[inline]
    pub fn try_new(s: &str) -> Option<Self> {
        let len = s.as_bytes().len();
        if len <= Self::CAPACITY {
            let stack = unsafe {
                // SAFETY: We've confirmed `len` is within size
                Self::new_unchecked(s)
            };
            Some(stack)
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// Calling this function with a string larger than `Self::CAPACITY` is undefined behavior.
    #[inline]
    pub unsafe fn new_unchecked(s: &str) -> Self {
        let len = s.as_bytes().len() as u8;
        debug_assert!(Self::CAPACITY <= Len::MAX.into());
        let buffer = StrBuffer::new_unchecked(s);
        Self { len, buffer }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        let len = self.len as usize;
        unsafe {
            // SAFETY: Constructors guarantee that `buffer[..len]` is a `str`,
            // and we don't mutate the data afterwards.
            self.buffer.as_str(len)
        }
    }

    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        let len = self.len as usize;
        unsafe {
            // SAFETY: Constructors guarantee that `buffer[..len]` is a `str`,
            // and we don't mutate the data afterwards.
            self.buffer.as_mut_str(len)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            assert!(self.is_char_boundary(new_len));
            self.len = new_len as u8;
        }
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_boxed_str(&self) -> Box<str> {
        Box::from(self.as_str())
    }
}

impl<const C: usize> Default for StackString<C> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const C: usize> std::ops::Deref for StackString<C> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const C: usize> Eq for StackString<C> {}

impl<'s, const C: usize> PartialEq<StackString<C>> for StackString<C> {
    #[inline]
    fn eq(&self, other: &StackString<C>) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<'s, const C: usize> PartialEq<str> for StackString<C> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl<'s, const C: usize> PartialEq<&'s str> for StackString<C> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl<'s, const C: usize> PartialEq<String> for StackString<C> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<const C: usize> Ord for StackString<C> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const C: usize> PartialOrd for StackString<C> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<const C: usize> std::hash::Hash for StackString<C> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<const C: usize> fmt::Debug for StackString<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const C: usize> fmt::Display for StackString<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<const C: usize> AsRef<str> for StackString<C> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const C: usize> AsRef<[u8]> for StackString<C> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const C: usize> AsRef<std::ffi::OsStr> for StackString<C> {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        (&**self).as_ref()
    }
}

impl<const C: usize> AsRef<std::path::Path> for StackString<C> {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl<const C: usize> std::borrow::Borrow<str> for StackString<C> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct StrBuffer<const C: usize>([u8; C]);

impl<const C: usize> StrBuffer<C> {
    pub(crate) const fn empty() -> Self {
        let array = [0; C];
        StrBuffer(array)
    }

    #[inline]
    pub(crate) unsafe fn new_unchecked(s: &str) -> Self {
        let len = s.as_bytes().len();
        debug_assert!(len <= C);
        let mut buffer = Self::default();
        buffer
            .0
            .get_unchecked_mut(..len)
            .copy_from_slice(s.as_bytes());
        buffer
    }

    #[inline]
    pub(crate) unsafe fn as_str(&self, len: usize) -> &str {
        let slice = self.0.get_unchecked(..len);
        std::str::from_utf8_unchecked(slice)
    }

    #[inline]
    pub(crate) unsafe fn as_mut_str(&mut self, len: usize) -> &mut str {
        let slice = self.0.get_unchecked_mut(..len);
        std::str::from_utf8_unchecked_mut(slice)
    }
}

impl<const C: usize> Default for StrBuffer<C> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        println!(
            "StackString: {}",
            std::mem::size_of::<StackString<CAPACITY>>()
        );
    }
}
