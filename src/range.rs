#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::point::Point;
    use std::convert::TryFrom;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Range {
        pub(crate) inner: tree_sitter::Range,
    }

    impl Range {
        #[inline]
        pub fn new(start_byte: u32, end_byte: u32, start_point: &Point, end_point: &Point) -> Self {
            let start_byte = start_byte as usize;
            let end_byte = end_byte as usize;
            let start_point = start_point.inner;
            let end_point = end_point.inner;
            tree_sitter::Range {
                start_byte,
                end_byte,
                start_point,
                end_point,
            }
            .into()
        }

        #[inline]
        pub fn end_byte(&self) -> u32 {
            u32::try_from(self.inner.end_byte).unwrap()
        }

        #[inline]
        pub fn end_point(&self) -> Point {
            let inner = self.inner.end_point;
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            u32::try_from(self.inner.start_byte).unwrap()
        }

        #[inline]
        pub fn start_point(&self) -> Point {
            let inner = self.inner.start_point;
            Point { inner }
        }
    }

    impl std::fmt::Debug for Range {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl Default for Range {
        fn default() -> Self {
            let start_byte = Default::default();
            let end_byte = Default::default();
            let start_position = &Default::default();
            let end_position = &Default::default();
            Self::new(start_byte, end_byte, start_position, end_position)
        }
    }

    impl From<tree_sitter::Range> for Range {
        #[inline]
        fn from(inner: tree_sitter::Range) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Range {
    }

    unsafe impl Send for Range {
    }

    unsafe impl Sync for Range {
    }

    impl Unpin for Range {
    }

    impl std::panic::UnwindSafe for Range {
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::point::Point;

    #[derive(Clone, Eq, PartialEq)]
    pub struct Range {
        pub(crate) inner: web_tree_sitter::Range,
    }

    impl Range {
        #[inline]
        pub fn new(start_byte: u32, end_byte: u32, start_point: &Point, end_point: &Point) -> Self {
            let start_point = &start_point.inner;
            let end_point = &end_point.inner;
            web_tree_sitter::Range::new(start_point, end_point, start_byte, end_byte).into()
        }

        #[inline]
        pub fn end_byte(&self) -> u32 {
            self.inner.end_index()
        }

        #[inline]
        pub fn end_point(&self) -> Point {
            let inner = self.inner.end_position();
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            self.inner.start_index()
        }

        #[inline]
        pub fn start_point(&self) -> Point {
            let inner = self.inner.start_position();
            Point { inner }
        }

        #[inline]
        fn spread(&self) -> (u32, u32, Point, Point) {
            (self.start_byte(), self.end_byte(), self.start_point(), self.end_point())
        }
    }

    impl std::fmt::Debug for Range {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl Default for Range {
        fn default() -> Self {
            let start_byte = Default::default();
            let end_byte = Default::default();
            let start_position = &Default::default();
            let end_position = &Default::default();
            Self::new(start_byte, end_byte, start_position, end_position)
        }
    }

    impl From<web_tree_sitter::Range> for Range {
        #[inline]
        fn from(inner: web_tree_sitter::Range) -> Self {
            Self { inner }
        }
    }

    impl Ord for Range {
        fn cmp(&self, that: &Self) -> std::cmp::Ordering {
            let this = self.spread();
            let that = that.spread();
            this.cmp(&that)
        }
    }

    impl PartialOrd<Range> for Range {
        fn partial_cmp(&self, that: &Self) -> Option<std::cmp::Ordering> {
            let this = self.spread();
            let that = that.spread();
            this.partial_cmp(&that)
        }
    }

    impl std::panic::RefUnwindSafe for Range {
    }

    unsafe impl Send for Range {
    }

    unsafe impl Sync for Range {
    }

    impl Unpin for Range {
    }

    impl std::panic::UnwindSafe for Range {
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
