/// A line number in a file. Internally the first line has index 1.
/// If it is 0 it means "no specific line" (used e.g. for implied directives).
/// When displayed, the first line is 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LineNumber(usize);


impl LineNumber {
    pub(crate) const ZERO: Self = Self(0);

    /// Create a LineNumber from a zero-based line index. I.e. if `zero_based`
    /// is `0` it means "the first line".
    pub(crate) fn from_zero_based(zero_based: usize) -> Self {
        // Ensure to panic on overflow.
        LineNumber(zero_based.strict_add(1))
    }

    pub(crate) fn from_one_based(one_based: usize) -> LineNumber {
        // You can't pass zero here. Use .none() for "no specific line".
        assert!(one_based > 0);
        LineNumber(one_based)
    }

    pub(crate) fn none() -> LineNumber {
        LineNumber(0)
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
