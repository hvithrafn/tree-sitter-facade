#[cfg(not(target_arch = "wasm32"))]
mod native {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: tree_sitter::IncludedRangesError,
    }
    unsafe impl Send for IncludedRangesError {
    }
    unsafe impl Sync for IncludedRangesError {
    }
    impl std::fmt::Display for IncludedRangesError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{:?}", self.inner)
        }
    }
    impl std::error::Error for IncludedRangesError {
    }
    impl From<tree_sitter::IncludedRangesError> for IncludedRangesError {
        #[inline]
        fn from(inner: tree_sitter::IncludedRangesError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: tree_sitter::QueryError,
    }
    unsafe impl Send for QueryError {
    }
    unsafe impl Sync for QueryError {
    }
    impl std::fmt::Display for QueryError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{:?}", self.inner)
        }
    }
    impl std::error::Error for QueryError {
    }
    impl From<tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: tree_sitter::LanguageError,
    }
    unsafe impl Send for LanguageError {
    }
    unsafe impl Sync for LanguageError {
    }
    impl std::fmt::Display for LanguageError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(fmt)
        }
    }
    impl std::error::Error for LanguageError {
    }
    impl From<tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: tree_sitter::LanguageError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct ParserError {
        pub(crate) inner: std::convert::Infallible,
    }
    unsafe impl Send for ParserError {
    }
    unsafe impl Sync for ParserError {
    }
    impl std::fmt::Display for ParserError {
        fn fmt(&self, _fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unreachable!()
        }
    }
    impl std::error::Error for ParserError {
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: js_sys::Error,
    }
    unsafe impl Send for IncludedRangesError {
    }
    unsafe impl Sync for IncludedRangesError {
    }
    impl std::fmt::Display for IncludedRangesError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for IncludedRangesError {
    }
    impl From<js_sys::Error> for IncludedRangesError {
        #[inline]
        fn from(inner: js_sys::Error) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: web_tree_sitter::LanguageError,
    }
    unsafe impl Send for LanguageError {
    }
    unsafe impl Sync for LanguageError {
    }
    impl std::fmt::Display for LanguageError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for LanguageError {
    }
    impl From<web_tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: web_tree_sitter::LanguageError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct ParserError {
        pub(crate) inner: web_tree_sitter::ParserError,
    }
    unsafe impl Send for ParserError {
    }
    unsafe impl Sync for ParserError {
    }
    impl std::fmt::Display for ParserError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for ParserError {
    }
    impl From<web_tree_sitter::ParserError> for ParserError {
        #[inline]
        fn from(inner: web_tree_sitter::ParserError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: web_tree_sitter::QueryError,
    }
    unsafe impl Send for QueryError {
    }
    unsafe impl Sync for QueryError {
    }
    impl std::fmt::Display for QueryError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for QueryError {
    }
    impl From<web_tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: web_tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;