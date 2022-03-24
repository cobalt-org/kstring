#[cfg(feature = "arc")]
pub type DefaultStr = crate::backend::ArcStr;
#[cfg(not(feature = "arc"))]
pub type DefaultStr = crate::backend::BoxedStr;

pub type BoxedStr = Box<str>;
static_assertions::assert_eq_size!(DefaultStr, BoxedStr);

pub type ArcStr = std::sync::Arc<str>;
static_assertions::assert_eq_size!(DefaultStr, ArcStr);

pub type RcStr = std::rc::Rc<str>;
static_assertions::assert_eq_size!(DefaultStr, RcStr);

pub trait StorageBackend: std::fmt::Debug + Clone + private::Sealed {
    fn from_str(other: &str) -> Self;
    fn from_string(other: String) -> Self;
    fn from_boxed_str(other: BoxedStr) -> Self;
    fn as_str(&self) -> &str;
}

impl StorageBackend for BoxedStr {
    #[inline]
    fn from_str(other: &str) -> Self {
        other.into()
    }

    #[inline]
    fn from_string(other: String) -> Self {
        other.into_boxed_str()
    }

    #[inline]
    fn from_boxed_str(other: BoxedStr) -> Self {
        other
    }

    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl StorageBackend for ArcStr {
    #[inline]
    fn from_str(other: &str) -> Self {
        other.into()
    }

    #[inline]
    fn from_string(other: String) -> Self {
        other.into_boxed_str().into()
    }

    #[inline]
    fn from_boxed_str(other: BoxedStr) -> Self {
        other.into()
    }

    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl StorageBackend for RcStr {
    #[inline]
    fn from_str(other: &str) -> Self {
        other.into()
    }

    #[inline]
    fn from_string(other: String) -> Self {
        other.into_boxed_str().into()
    }

    #[inline]
    fn from_boxed_str(other: BoxedStr) -> Self {
        other.into()
    }

    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

pub(crate) mod private {
    pub trait Sealed {}
    impl Sealed for super::BoxedStr {}
    impl Sealed for super::ArcStr {}
    impl Sealed for super::RcStr {}
}
