use std::fmt;

pub(crate) type Len = u8;

#[derive(Copy, Clone)]
pub struct StackString<const CAPACITY: usize> {
    len: Len,
    buffer: StrBuffer<CAPACITY>,
}

impl<const CAPACITY: usize> StackString<CAPACITY> {
    pub const CAPACITY: usize = CAPACITY;
    pub const EMPTY: Self = Self::empty();

    const fn empty() -> Self {
        Self {
            len: 0,
            buffer: StrBuffer::empty(),
        }
    }

    #[inline]
    pub fn try_new(s: impl AsRef<str>) -> Option<Self> {
        let s = s.as_ref();
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
    pub unsafe fn new_unchecked(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
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
}

impl<const CAPACITY: usize> Default for StackString<CAPACITY> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const CAPACITY: usize> std::ops::Deref for StackString<CAPACITY> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const CAPACITY: usize> Eq for StackString<CAPACITY> {}

impl<const C1: usize, const C2: usize> PartialEq<StackString<C1>> for StackString<C2> {
    #[inline]
    fn eq(&self, other: &StackString<C1>) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<const CAPACITY: usize> PartialEq<str> for StackString<CAPACITY> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl<'s, const CAPACITY: usize> PartialEq<&'s str> for StackString<CAPACITY> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl<const CAPACITY: usize> PartialEq<String> for StackString<CAPACITY> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<const CAPACITY: usize> Ord for StackString<CAPACITY> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const C1: usize, const C2: usize> PartialOrd<StackString<C1>> for StackString<C2> {
    #[inline]
    fn partial_cmp(&self, other: &StackString<C1>) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<const CAPACITY: usize> PartialOrd<str> for StackString<CAPACITY> {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl<'s, const CAPACITY: usize> PartialOrd<&'s str> for StackString<CAPACITY> {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl<const CAPACITY: usize> PartialOrd<String> for StackString<CAPACITY> {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<const CAPACITY: usize> std::hash::Hash for StackString<CAPACITY> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<const CAPACITY: usize> fmt::Debug for StackString<CAPACITY> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const CAPACITY: usize> fmt::Display for StackString<CAPACITY> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<const CAPACITY: usize> AsRef<str> for StackString<CAPACITY> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const CAPACITY: usize> AsRef<[u8]> for StackString<CAPACITY> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const CAPACITY: usize> AsRef<std::ffi::OsStr> for StackString<CAPACITY> {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        (&**self).as_ref()
    }
}

impl<const CAPACITY: usize> AsRef<std::path::Path> for StackString<CAPACITY> {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl<const CAPACITY: usize> std::borrow::Borrow<str> for StackString<CAPACITY> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct StrBuffer<const CAPACITY: usize>([u8; CAPACITY]);

impl<const CAPACITY: usize> StrBuffer<CAPACITY> {
    pub(crate) const fn empty() -> Self {
        let array = [0; CAPACITY];
        StrBuffer(array)
    }

    #[inline]
    pub(crate) unsafe fn new_unchecked(s: &str) -> Self {
        let len = s.as_bytes().len();
        debug_assert!(len <= CAPACITY);
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

impl<const CAPACITY: usize> Default for StrBuffer<CAPACITY> {
    fn default() -> Self {
        Self::empty()
    }
}
