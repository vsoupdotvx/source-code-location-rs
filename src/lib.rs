pub use source_code_location_derive::location;

/// Represents a location in source code.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    string: &'static &'static str,
}

impl Location {
    #[doc(hidden)]
    pub fn new(string: &'static &'static str) -> Self {
        Self { string }
    }
    
    /// Gets the file path of this location. This may be an invalid
    /// [`Path`](std::path::Path)!
    pub fn file(&self) -> &'static str {
        self.string.rsplit(':').nth(2).unwrap()
    }
    
    /// Gets the line number of this location.
    pub fn line(&self) -> usize {
        self.string.rsplit(':').nth(1).unwrap().parse().unwrap()
    }
    
    /// Gets the column number of this location.
    pub fn column(&self) -> usize {
        self.string.rsplit(':').nth(0).unwrap().parse().unwrap()
    }
    
    /// Gets the entire location string, formatted as {file}:{line}:{column}.
    pub fn location_str(&self) -> &'static str {
        self.string
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string)
    }
}

impl std::error::Error for Location {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[cfg(test)]
pub mod tests {
    use source_code_location_derive::location_test;
    #[test]
    fn test_all() {
        assert_eq!("src/lib.rs:60:54", format!("{}", location_test!()));
        assert_eq!("src/lib.rs:61:40", location_test!().location_str());
    }
    #[test]
    fn file_test() {
        assert_eq!("src/lib.rs", location_test!().file());
    }
    #[test]
    fn line_test() {
        assert_eq!(69, location_test!().line());
    }
    #[test]
    fn column_test() {
        assert_eq!(24, location_test!().column());
    }
}
