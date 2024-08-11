#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReverseStr<'a>(&'a str);

impl<'a> std::cmp::PartialOrd for ReverseStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> std::cmp::Ord for ReverseStr<'a> {
    
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        lexical_sort::natural_lexical_cmp(&self.0, &other.0)
    }
}


impl<'a> std::fmt::Display for ReverseStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<'a> From<&'a String> for ReverseStr<'a> {
    fn from(value: &'a String) -> Self {
        let x: &str = value;
        Self(x)
    }
}