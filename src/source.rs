use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        debug_assert!(start <= end);

        Span { start, end }
    }

    pub fn cover(a: Span, b: Span) -> Span {
        let start = a.start().min(b.start());
        let end = a.end().max(b.end());

        Span::new(start, end)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

/// Position in source code.
///
/// The position is represented by its character offset, line, and column. The
/// offset is zero-based, the line and column are one-based.
///
/// The `Eq`, `PartialEq`, `Ord`, and `PartialOrd` traits are implemented in
/// terms of the character offset. This reflects an assumption that the line and
/// column are derived information.
///
/// # Example
///
/// ```
/// use sari::SourcePos;
///
/// let pos = SourcePos::new(69, 5, 7); // offset 69, line 5, column 7
///
/// assert_eq!(pos.offset(), 69);
/// assert_eq!(pos.line(), 5);
/// assert_eq!(pos.column(), 7);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct SourcePos {
    offset: usize,
    line: usize,
    column: usize,
}

impl SourcePos {
    /// Creates a new `SourcePos` with specified character offset, line, and
    /// column.
    ///
    /// # Example
    ///
    /// ```
    /// use sari::SourcePos;
    ///
    /// let pos = SourcePos::new(69, 5, 7); // offset 69, line 5, column 7
    ///
    /// assert_eq!(pos.offset(), 69);
    /// assert_eq!(pos.line(), 5);
    /// assert_eq!(pos.column(), 7);
    /// ```
    pub fn new(offset: usize, line: usize, column: usize) -> SourcePos {
        SourcePos {
            offset,
            line,
            column,
        }
    }

    /// Returns the character offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the line.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Returns the column.
    pub fn column(&self) -> usize {
        self.column
    }
}

impl Eq for SourcePos {}

impl PartialEq for SourcePos {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Ord for SourcePos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl PartialOrd for SourcePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for SourcePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Span in source code.
///
/// The span is represented by its start and end position. The start position is
/// inclusive, the end position is exclusive.
///
/// The end position is alwyas greater or equal to the start position (when the
/// positions are equal, the span is empty).
///
/// # Example
///
/// ```
/// use sari::{SourcePos, SourceSpan};
///
/// let span = SourceSpan::new(
///     SourcePos::new(69, 5, 7), // offset 69, line 5, column 7
///     SourcePos::new(74, 5, 12), // offset 74, line 5, column 12
/// );
///
/// assert_eq!(span.start(), SourcePos::new(69, 5, 7));
/// assert_eq!(span.end(), SourcePos::new(74, 5, 12));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SourceSpan {
    start: SourcePos,
    end: SourcePos,
}

impl SourceSpan {
    /// Creates a new `SourceSpan` with specified start and end position.
    ///
    /// # Panics
    ///
    /// Panics if `end < start`.
    ///
    /// # Example
    ///
    /// ```
    /// use sari::{SourcePos, SourceSpan};
    ///
    /// let span = SourceSpan::new(
    ///     SourcePos::new(69, 5, 7), // offset 69, line 5, column 7
    ///     SourcePos::new(74, 5, 12), // offset 74, line 5, column 12
    /// );
    ///
    /// assert_eq!(span.start(), SourcePos::new(69, 5, 7));
    /// assert_eq!(span.end(), SourcePos::new(74, 5, 12));
    /// ```
    ///
    /// ```should_panic
    /// use sari::{SourcePos, SourceSpan};
    ///
    /// let span = SourceSpan::new(
    ///     SourcePos::new(74, 5, 12), // offset 74, line 5, column 12
    ///     SourcePos::new(69, 5, 7), // offset 69, line 5, column 7
    /// );
    /// ```
    pub fn new(start: SourcePos, end: SourcePos) -> SourceSpan {
        assert!(start <= end);

        SourceSpan { start, end }
    }

    /// Returns the start position.
    pub fn start(&self) -> SourcePos {
        self.start
    }

    /// Returns the end position.
    pub fn end(&self) -> SourcePos {
        self.end
    }
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

pub struct SourceMap {
    line_starts: Vec<usize>,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap {
            // There is always at least one line, starting at offset 0. This is
            // the case even for an empty input.
            line_starts: vec![0],
        }
    }

    pub fn add_line_start(&mut self, pos: usize) {
        debug_assert!(self.line_starts[self.line_starts.len() - 1] < pos);

        self.line_starts.push(pos);
    }

    pub fn map_span(&self, span: Span) -> SourceSpan {
        let start = self.map_pos(span.start());
        let end = self.map_pos(span.end());

        SourceSpan::new(start, end)
    }

    fn map_pos(&self, pos: usize) -> SourcePos {
        // The algorithm below is binary search, modified in two ways:
        //
        //   1. Instead of tracking the lower and upper bound of the search
        //      interval, we track its base and size. This works better with
        //      unsigned integers.
        //
        //   2. We don't look for an exact match, but the greatest value less
        //      or equal to the target one. We guarantee there always is one.

        let mut base = 0;
        let mut size = self.line_starts.len();
        let mut index = 0;

        while size > 0 {
            let half = size / 2;
            let mid = base + half;

            if self.line_starts[mid] <= pos {
                index = mid;
                base = mid + 1;
            }
            size = size - half - 1;
        }

        SourcePos::new(pos, index + 1, pos - self.line_starts[index] + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_cover_works() {
        let span_1 = Span::new(0, 4);
        let span_2 = Span::new(4, 8);

        assert_eq!(Span::cover(span_1, span_2), Span::new(0, 8));
        assert_eq!(Span::cover(span_2, span_1), Span::new(0, 8));
    }

    #[test]
    fn source_pos_eq_works() {
        let pos_1 = SourcePos::new(4, 1, 5);
        let pos_2 = SourcePos::new(5, 1, 6);

        assert!(pos_1.eq(&pos_1));
        assert!(!pos_1.eq(&pos_2));
    }

    #[test]
    fn source_pos_cmp_works() {
        let pos_1 = SourcePos::new(4, 1, 5);
        let pos_2 = SourcePos::new(5, 1, 6);
        let pos_3 = SourcePos::new(6, 1, 7);

        assert_eq!(pos_2.cmp(&pos_1), Ordering::Greater);
        assert_eq!(pos_2.cmp(&pos_2), Ordering::Equal);
        assert_eq!(pos_2.cmp(&pos_3), Ordering::Less);
    }

    #[test]
    fn source_pos_partial_cmp_works() {
        let pos_1 = SourcePos::new(4, 1, 5);
        let pos_2 = SourcePos::new(5, 1, 6);
        let pos_3 = SourcePos::new(6, 1, 7);

        assert_eq!(pos_2.partial_cmp(&pos_1), Some(Ordering::Greater));
        assert_eq!(pos_2.partial_cmp(&pos_2), Some(Ordering::Equal));
        assert_eq!(pos_2.partial_cmp(&pos_3), Some(Ordering::Less));
    }

    #[test]
    fn source_pos_format_works() {
        let pos = SourcePos::new(4, 1, 5);

        assert_eq!(pos.to_string(), "1:5");
    }

    #[test]
    fn source_span_format_works() {
        let span = SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(8, 2, 3));

        assert_eq!(span.to_string(), "1:5-2:3");
    }

    #[test]
    fn source_map_mapping() {
        let mut source_map = SourceMap::new();
        source_map.add_line_start(4);
        source_map.add_line_start(8);

        // line 1
        assert_eq!(
            source_map.map_span(Span::new(0, 2)),
            SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(2, 1, 3))
        );

        // line 2
        assert_eq!(
            source_map.map_span(Span::new(4, 6)),
            SourceSpan::new(SourcePos::new(4, 2, 1), SourcePos::new(6, 2, 3))
        );

        // line 3
        assert_eq!(
            source_map.map_span(Span::new(8, 10)),
            SourceSpan::new(SourcePos::new(8, 3, 1), SourcePos::new(10, 3, 3))
        );
    }
}
