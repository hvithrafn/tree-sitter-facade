#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::convert::TryFrom;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Point {
        pub(crate) inner: tree_sitter::Point,
    }

    impl Point {
        #[inline]
        pub fn new(row: u32, column: u32) -> Self {
            let row = row as usize;
            let column = column as usize;
            tree_sitter::Point { row, column }.into()
        }

        #[inline]
        pub fn column(&self) -> u32 {
            u32::try_from(self.inner.column).unwrap()
        }

        #[inline]
        pub fn row(&self) -> u32 {
            u32::try_from(self.inner.row).unwrap()
        }
    }

    impl std::fmt::Debug for Point {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl std::fmt::Display for Point {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.inner, fmt)
        }
    }

    impl Default for Point {
        fn default() -> Self {
            let row = Default::default();
            let column = Default::default();
            Self::new(row, column)
        }
    }

    impl From<tree_sitter::Point> for Point {
        #[inline]
        fn from(inner: tree_sitter::Point) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Point {
    }

    unsafe impl Send for Point {
    }

    unsafe impl Sync for Point {
    }

    impl Unpin for Point {
    }

    impl std::panic::UnwindSafe for Point {
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Clone, Eq, PartialEq)]
    pub struct Point {
        pub(crate) inner: web_tree_sitter::Point,
    }

    impl Point {
        #[inline]
        pub fn new(row: u32, column: u32) -> Self {
            web_tree_sitter::Point::new(row, column).into()
        }

        #[inline]
        pub fn column(&self) -> u32 {
            self.inner.column()
        }

        #[inline]
        pub fn row(&self) -> u32 {
            self.inner.row()
        }

        #[inline]
        fn spread(&self) -> (u32, u32) {
            (self.row(), self.column())
        }
    }

    impl std::fmt::Debug for Point {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl std::fmt::Display for Point {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "({}, {})", self.row(), self.column())
        }
    }

    impl Default for Point {
        fn default() -> Self {
            let row = Default::default();
            let column = Default::default();
            Self::new(row, column)
        }
    }

    impl From<web_tree_sitter::Point> for Point {
        #[inline]
        fn from(inner: web_tree_sitter::Point) -> Self {
            Self { inner }
        }
    }

    impl std::hash::Hash for Point {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            let this = self.spread();
            this.hash(state)
        }
    }

    impl Ord for Point {
        fn cmp(&self, that: &Self) -> std::cmp::Ordering {
            let this = self.spread();
            let that = that.spread();
            this.cmp(&that)
        }
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, that: &Point) -> Option<std::cmp::Ordering> {
            let this = self.spread();
            let that = that.spread();
            this.partial_cmp(&that)
        }
    }

    impl std::panic::RefUnwindSafe for Point {
    }

    unsafe impl Send for Point {
    }

    unsafe impl Sync for Point {
    }

    impl Unpin for Point {
    }

    impl std::panic::UnwindSafe for Point {
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
