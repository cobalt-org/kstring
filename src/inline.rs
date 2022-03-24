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
pub(crate) struct InlineString<const C: usize> {
    len: Len,
    buffer: StrBuffer<C>,
}

impl<const C: usize> InlineString<C> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(s: &str) -> Self {
        let len = s.as_bytes().len() as u8;
        debug_assert!(C <= Len::MAX.into());
        let buffer = StrBuffer::new_unchecked(s);
        Self { len, buffer }
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        let len = self.len as usize;
        unsafe {
            // SAFETY: Constructors guarantee that `buffer[..len]` is a `str`,
            // and we don't mutate the data afterwards.
            self.buffer.as_str(len)
        }
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_boxed_str(&self) -> Box<str> {
        Box::from(self.as_str())
    }
}

impl<const C: usize> fmt::Debug for InlineString<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct StrBuffer<const C: usize>([u8; C]);

impl<const C: usize> StrBuffer<C> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(s: &str) -> Self {
        let len = s.as_bytes().len();
        debug_assert!(len <= C);
        let mut array = [0; C];
        array.get_unchecked_mut(..len).copy_from_slice(s.as_bytes());
        Self(array)
    }

    #[inline]
    pub(crate) unsafe fn as_str(&self, len: usize) -> &str {
        let slice = self.0.get_unchecked(..len);
        std::str::from_utf8_unchecked(slice)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        println!(
            "InlineString: {}",
            std::mem::size_of::<InlineString<CAPACITY>>()
        );
    }
}
