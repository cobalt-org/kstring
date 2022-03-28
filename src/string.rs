use std::{borrow::Cow, fmt};

use crate::stack::StackString;
use crate::KStringCow;
use crate::KStringRef;

pub(crate) type StdString = std::string::String;
type BoxedStr = Box<str>;
#[cfg(feature = "arc")]
pub(crate) type OwnedStr = std::sync::Arc<str>;
#[cfg(not(feature = "arc"))]
pub(crate) type OwnedStr = Box<str>;

/// A UTF-8 encoded, immutable string.
#[derive(Clone)]
#[repr(transparent)]
pub struct KString {
    inner: KStringInner,
}

impl KString {
    pub const EMPTY: Self = KString::from_static("");

    /// Create a new empty `KString`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Create an owned `KString`.
    #[inline]
    #[must_use]
    pub fn from_boxed(other: BoxedStr) -> Self {
        Self {
            inner: KStringInner::from_boxed(other),
        }
    }

    /// Create an owned `KString`.
    #[inline]
    #[must_use]
    pub fn from_string(other: StdString) -> Self {
        Self {
            inner: KStringInner::from_string(other),
        }
    }

    /// Create an owned `KString` optimally from a reference.
    #[inline]
    #[must_use]
    pub fn from_ref(other: &str) -> Self {
        Self {
            inner: KStringInner::from_ref(other),
        }
    }

    /// Create a reference to a `'static` data.
    #[inline]
    #[must_use]
    pub const fn from_static(other: &'static str) -> Self {
        Self {
            inner: KStringInner::from_static(other),
        }
    }

    /// Create an inline string, if possible
    #[inline]
    #[must_use]
    pub fn try_inline(other: &str) -> Option<Self> {
        KStringInner::try_inline(other).map(|inner| Self { inner })
    }

    /// Get a reference to the `KString`.
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> KStringRef<'_> {
        self.inner.as_ref()
    }

    /// Extracts a string slice containing the entire `KString`.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Convert to a mutable string type, cloning the data if necessary.
    #[inline]
    #[must_use]
    pub fn into_string(self) -> StdString {
        String::from(self.into_boxed_str())
    }

    /// Convert to a mutable string type, cloning the data if necessary.
    #[inline]
    #[must_use]
    pub fn into_boxed_str(self) -> BoxedStr {
        self.inner.into_boxed_str()
    }

    /// Convert to a Cow str
    #[inline]
    #[must_use]
    pub fn into_cow_str(self) -> Cow<'static, str> {
        self.inner.into_cow_str()
    }
}

impl std::ops::Deref for KString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Eq for KString {}

impl<'s> PartialEq<KString> for KString {
    #[inline]
    fn eq(&self, other: &KString) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<'s> PartialEq<str> for KString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl<'s> PartialEq<&'s str> for KString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl<'s> PartialEq<String> for KString {
    #[inline]
    fn eq(&self, other: &StdString) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl Ord for KString {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for KString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl std::hash::Hash for KString {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl fmt::Debug for KString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Display for KString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for KString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for KString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<std::ffi::OsStr> for KString {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        (&**self).as_ref()
    }
}

impl AsRef<std::path::Path> for KString {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl std::borrow::Borrow<str> for KString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Default for KString {
    #[inline]
    fn default() -> Self {
        Self::from_static("")
    }
}

impl<'s> From<KStringRef<'s>> for KString {
    #[inline]
    fn from(other: KStringRef<'s>) -> Self {
        other.to_owned()
    }
}

impl<'s> From<&'s KStringRef<'s>> for KString {
    #[inline]
    fn from(other: &'s KStringRef<'s>) -> Self {
        other.to_owned()
    }
}

impl<'s> From<KStringCow<'s>> for KString {
    #[inline]
    fn from(other: KStringCow<'s>) -> Self {
        other.into_owned()
    }
}

impl<'s> From<&'s KStringCow<'s>> for KString {
    #[inline]
    fn from(other: &'s KStringCow<'s>) -> Self {
        other.clone().into_owned()
    }
}

impl From<StdString> for KString {
    #[inline]
    fn from(other: StdString) -> Self {
        Self::from_string(other)
    }
}

impl<'s> From<&'s StdString> for KString {
    #[inline]
    fn from(other: &'s StdString) -> Self {
        Self::from_ref(other)
    }
}

impl From<BoxedStr> for KString {
    #[inline]
    fn from(other: BoxedStr) -> Self {
        Self::from_boxed(other)
    }
}

impl<'s> From<&'s BoxedStr> for KString {
    #[inline]
    fn from(other: &'s BoxedStr) -> Self {
        Self::from_ref(other)
    }
}

impl From<&'static str> for KString {
    #[inline]
    fn from(other: &'static str) -> Self {
        Self::from_static(other)
    }
}

impl std::str::FromStr for KString {
    type Err = std::convert::Infallible;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(KString::from_ref(s))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for KString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for KString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor)
    }
}

#[cfg(feature = "serde")]
struct StringVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = KString;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(KString::from_ref(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(KString::from_string(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => Ok(KString::from_ref(s)),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(v),
                &self,
            )),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(KString::from_string(s)),
            Err(e) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }
}

use inner::KStringInner;

#[cfg(not(feature = "unsafe"))]
mod inner {
    use super::*;

    pub(super) enum KStringInner {
        Singleton(&'static str),
        Inline(StackString<CAPACITY>),
        Owned(OwnedStr),
    }

    impl KStringInner {
        #[inline]
        pub(super) fn from_boxed(other: BoxedStr) -> Self {
            #[allow(clippy::useless_conversion)]
            Self::Owned(OwnedStr::from(other))
        }

        #[inline]
        pub(super) fn from_string(other: StdString) -> Self {
            if (0..=CAPACITY).contains(&other.len()) {
                let inline = { StackString::new(other.as_str()) };
                Self::Inline(inline)
            } else {
                #[allow(clippy::useless_conversion)]
                Self::Owned(OwnedStr::from(other.into_boxed_str()))
            }
        }

        #[inline]
        pub(super) fn from_ref(other: &str) -> Self {
            if (0..=CAPACITY).contains(&other.len()) {
                let inline = { StackString::new(other) };
                Self::Inline(inline)
            } else {
                #[allow(clippy::useless_conversion)]
                Self::Owned(OwnedStr::from(other))
            }
        }

        /// Create a reference to a `'static` data.
        #[inline]
        pub const fn from_static(other: &'static str) -> Self {
            Self::Singleton(other)
        }

        #[inline]
        pub fn try_inline(other: &str) -> Option<Self> {
            StackString::try_new(other).map(Self::Inline)
        }

        #[inline]
        pub(super) fn as_ref(&self) -> KStringRef<'_> {
            match self {
                Self::Singleton(s) => KStringRef::from_static(s),
                Self::Inline(s) => KStringRef::from_ref(s.as_str()),
                Self::Owned(s) => KStringRef::from_ref(s),
            }
        }

        #[inline]
        pub(super) fn as_str(&self) -> &str {
            match self {
                Self::Singleton(s) => s,
                Self::Inline(s) => s.as_str(),
                Self::Owned(s) => s,
            }
        }

        #[inline]
        pub(super) fn into_boxed_str(self) -> BoxedStr {
            match self {
                Self::Singleton(s) => BoxedStr::from(s),
                Self::Inline(s) => BoxedStr::from(s.as_str()),
                Self::Owned(s) => BoxedStr::from(s.as_ref()),
            }
        }

        /// Convert to a Cow str
        #[inline]
        pub(super) fn into_cow_str(self) -> Cow<'static, str> {
            match self {
                Self::Singleton(s) => Cow::Borrowed(s),
                Self::Inline(s) => Cow::Owned(s.as_str().into()),
                Self::Owned(s) => Cow::Owned(s.as_ref().into()),
            }
        }
    }

    // Explicit to avoid inlining which cuts clone times in half.
    //
    // An automatically derived `clone()` has 10ns overhead while the explicit `Deref`/`as_str` has
    // none of that.  Being explicit and removing the `#[inline]` attribute dropped the overhead to
    // 5ns.
    //
    // My only guess is that the `clone()` calls we delegate to are just that much bigger than
    // `as_str()` that, when combined with a jump table, is blowing the icache, slowing things down.
    impl Clone for KStringInner {
        fn clone(&self) -> Self {
            match self {
                Self::Singleton(s) => Self::Singleton(s),
                Self::Inline(s) => Self::Inline(*s),
                Self::Owned(s) => Self::Owned(s.clone()),
            }
        }
    }

    #[allow(unused)]
    const LEN_SIZE: usize = std::mem::size_of::<crate::stack::Len>();

    #[allow(unused)]
    const TAG_SIZE: usize = std::mem::size_of::<u8>();

    #[allow(unused)]
    const MAX_CAPACITY: usize =
        std::mem::size_of::<crate::string::StdString>() - TAG_SIZE - LEN_SIZE;

    // Performance seems to slow down when trying to occupy all of the padding left by `String`'s
    // discriminant.  The question is whether faster len=1-16 "allocations" outweighs going to the heap
    // for len=17-22.
    #[allow(unused)]
    const ALIGNED_CAPACITY: usize = std::mem::size_of::<crate::string::OwnedStr>() - LEN_SIZE;

    #[cfg(feature = "max_inline")]
    const CAPACITY: usize = MAX_CAPACITY;
    #[cfg(not(feature = "max_inline"))]
    const CAPACITY: usize = ALIGNED_CAPACITY;
}

#[cfg(feature = "unsafe")]
mod inner {
    use super::*;

    pub(super) union KStringInner {
        tag: TagVariant,
        singleton: SingletonVariant,
        owned: std::mem::ManuallyDrop<OwnedVariant>,
        inline: InlineVariant,
    }

    impl KStringInner {
        #[inline]
        pub(super) fn from_boxed(other: BoxedStr) -> Self {
            #[allow(clippy::useless_conversion)]
            let payload = OwnedStr::from(other);
            Self {
                owned: std::mem::ManuallyDrop::new(OwnedVariant::new(payload)),
            }
        }

        #[inline]
        pub(super) fn from_string(other: StdString) -> Self {
            if (0..=CAPACITY).contains(&other.len()) {
                let payload = unsafe {
                    // SAFETY: range check ensured this is always safe
                    StackString::new_unchecked(other.as_str())
                };
                Self {
                    inline: InlineVariant::new(payload),
                }
            } else {
                Self::from_boxed(other.into_boxed_str())
            }
        }

        #[inline]
        pub(super) fn from_ref(other: &str) -> Self {
            if (0..=CAPACITY).contains(&other.len()) {
                let payload = unsafe {
                    // SAFETY: range check ensured this is always safe
                    StackString::new_unchecked(other)
                };
                Self {
                    inline: InlineVariant::new(payload),
                }
            } else {
                #[allow(clippy::useless_conversion)]
                let payload = OwnedStr::from(other);
                Self {
                    owned: std::mem::ManuallyDrop::new(OwnedVariant::new(payload)),
                }
            }
        }

        /// Create a reference to a `'static` data.
        #[inline]
        pub const fn from_static(other: &'static str) -> Self {
            Self {
                singleton: SingletonVariant::new(other),
            }
        }

        #[inline]
        pub fn try_inline(other: &str) -> Option<Self> {
            StackString::try_new(other).map(|inline| Self {
                inline: InlineVariant::new(inline),
            })
        }

        #[inline]
        pub(super) fn as_ref(&self) -> KStringRef<'_> {
            let tag = self.tag();
            unsafe {
                // SAFETY: `tag` ensures access to correct variant
                if tag.is_singleton() {
                    KStringRef::from_static(self.singleton.payload)
                } else if tag.is_owned() {
                    KStringRef::from_ref(self.owned.payload.as_ref())
                } else {
                    debug_assert!(tag.is_inline());
                    KStringRef::from_ref(self.inline.payload.as_str())
                }
            }
        }

        #[inline]
        pub(super) fn as_str(&self) -> &str {
            let tag = self.tag();
            unsafe {
                // SAFETY: `tag` ensures access to correct variant
                if tag.is_singleton() {
                    self.singleton.payload
                } else if tag.is_owned() {
                    self.owned.payload.as_ref()
                } else {
                    debug_assert!(tag.is_inline());
                    self.inline.payload.as_str()
                }
            }
        }

        #[inline]
        pub(super) fn into_boxed_str(self) -> BoxedStr {
            let tag = self.tag();
            unsafe {
                // SAFETY: `tag` ensures access to correct variant
                if tag.is_singleton() {
                    BoxedStr::from(self.singleton.payload)
                } else if tag.is_owned() {
                    BoxedStr::from(self.owned.payload.as_ref())
                } else {
                    debug_assert!(tag.is_inline());
                    BoxedStr::from(self.inline.payload.as_ref())
                }
            }
        }

        /// Convert to a Cow str
        #[inline]
        pub(super) fn into_cow_str(self) -> Cow<'static, str> {
            let tag = self.tag();
            unsafe {
                // SAFETY: `tag` ensures access to correct variant
                if tag.is_singleton() {
                    Cow::Borrowed(self.singleton.payload)
                } else if tag.is_owned() {
                    Cow::Owned(self.owned.payload.as_ref().into())
                } else {
                    debug_assert!(tag.is_inline());
                    Cow::Owned(self.inline.payload.as_str().into())
                }
            }
        }

        #[inline]
        fn tag(&self) -> Tag {
            unsafe {
                // SAFETY: `tag` is in the same spot in each variant
                self.tag.tag
            }
        }
    }

    // Explicit to avoid inlining which cuts clone times in half.
    //
    // An automatically derived `clone()` has 10ns overhead while the explicit `Deref`/`as_str` has
    // none of that.  Being explicit and removing the `#[inline]` attribute dropped the overhead to
    // 5ns.
    //
    // My only guess is that the `clone()` calls we delegate to are just that much bigger than
    // `as_str()` that, when combined with a jump table, is blowing the icache, slowing things down.
    impl Clone for KStringInner {
        fn clone(&self) -> Self {
            let tag = self.tag();
            if tag.is_owned() {
                unsafe {
                    // SAFETY: `tag` ensures access to correct variant
                    Self {
                        owned: std::mem::ManuallyDrop::new(OwnedVariant::new(
                            self.owned.payload.clone(),
                        )),
                    }
                }
            } else {
                unsafe {
                    // SAFETY: `tag` ensures access to correct variant
                    // SAFETY: non-owned types are copyable
                    std::mem::transmute_copy(self)
                }
            }
        }
    }

    impl Drop for KStringInner {
        fn drop(&mut self) {
            let tag = self.tag();
            if tag.is_owned() {
                unsafe {
                    // SAFETY: `tag` ensures we are using the right variant
                    std::mem::ManuallyDrop::drop(&mut self.owned)
                }
            }
        }
    }

    #[allow(unused)]
    const LEN_SIZE: usize = std::mem::size_of::<crate::stack::Len>();

    #[allow(unused)]
    const TAG_SIZE: usize = std::mem::size_of::<Tag>();

    #[allow(unused)]
    const PAYLOAD_SIZE: usize = std::mem::size_of::<crate::string::OwnedStr>();
    type Payload = Padding<PAYLOAD_SIZE>;

    #[allow(unused)]
    const TARGET_SIZE: usize = std::mem::size_of::<Target>();
    type Target = crate::string::StdString;

    #[allow(unused)]
    const MAX_CAPACITY: usize = TARGET_SIZE - LEN_SIZE - TAG_SIZE;

    // Performance seems to slow down when trying to occupy all of the padding left by `String`'s
    // discriminant.  The question is whether faster len=1-16 "allocations" outweighs going to the heap
    // for len=17-22.
    #[allow(unused)]
    const ALIGNED_CAPACITY: usize = PAYLOAD_SIZE - LEN_SIZE;

    #[cfg(feature = "max_inline")]
    const CAPACITY: usize = MAX_CAPACITY;
    #[cfg(not(feature = "max_inline"))]
    const CAPACITY: usize = ALIGNED_CAPACITY;

    const PAYLOAD_PAD_SIZE: usize = TARGET_SIZE - PAYLOAD_SIZE - TAG_SIZE;
    const INLINE_PAD_SIZE: usize = TARGET_SIZE - CAPACITY - LEN_SIZE - TAG_SIZE;

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct TagVariant {
        payload: Payload,
        pad: Padding<PAYLOAD_PAD_SIZE>,
        tag: Tag,
    }
    static_assertions::assert_eq_size!(Target, TagVariant);

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct SingletonVariant {
        payload: &'static str,
        pad: Padding<PAYLOAD_PAD_SIZE>,
        tag: Tag,
    }
    static_assertions::assert_eq_size!(Payload, &'static str);
    static_assertions::assert_eq_size!(Target, SingletonVariant);

    impl SingletonVariant {
        #[inline]
        const fn new(payload: &'static str) -> Self {
            Self {
                payload,
                pad: Padding::new(),
                tag: Tag::SINGLETON,
            }
        }
    }

    impl std::fmt::Debug for SingletonVariant {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.payload.fmt(f)
        }
    }

    #[derive(Clone)]
    #[repr(C)]
    struct OwnedVariant {
        payload: OwnedStr,
        pad: Padding<PAYLOAD_PAD_SIZE>,
        tag: Tag,
    }
    static_assertions::assert_eq_size!(Payload, std::mem::ManuallyDrop<crate::string::OwnedStr>);
    static_assertions::assert_eq_size!(Target, OwnedVariant);

    impl OwnedVariant {
        #[inline]
        const fn new(payload: OwnedStr) -> Self {
            Self {
                payload,
                pad: Padding::new(),
                tag: Tag::OWNED,
            }
        }
    }

    impl std::fmt::Debug for OwnedVariant {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.payload.fmt(f)
        }
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct InlineVariant {
        payload: StackString<CAPACITY>,
        pad: Padding<INLINE_PAD_SIZE>,
        tag: Tag,
    }
    static_assertions::assert_eq_size!(Target, InlineVariant);

    impl InlineVariant {
        #[inline]
        const fn new(payload: StackString<CAPACITY>) -> Self {
            Self {
                payload,
                pad: Padding::new(),
                tag: Tag::INLINE,
            }
        }
    }

    impl std::fmt::Debug for InlineVariant {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.payload.fmt(f)
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    struct Tag(u8);

    impl Tag {
        const SINGLETON: Tag = Tag(0);
        const OWNED: Tag = Tag(u8::MAX);
        const INLINE: Tag = Tag(1);

        #[inline]
        const fn is_singleton(self) -> bool {
            self.0 == Self::SINGLETON.0
        }

        #[inline]
        const fn is_owned(self) -> bool {
            self.0 == Self::OWNED.0
        }

        #[inline]
        const fn is_inline(self) -> bool {
            !self.is_singleton() && !self.is_owned()
        }
    }

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    struct Padding<const L: usize>([std::mem::MaybeUninit<u8>; L]);

    impl<const L: usize> Padding<L> {
        const fn new() -> Self {
            let padding = unsafe {
                // SAFETY: Padding, never actually used
                std::mem::MaybeUninit::uninit().assume_init()
            };
            Self(padding)
        }
    }

    impl<const L: usize> Default for Padding<L> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        println!("KString: {}", std::mem::size_of::<KString>());
    }
}
