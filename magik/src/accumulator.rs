use crate::Renderable;

/// A structure that accumulates multiple templates and renders them concatenated
/// 
/// This allows you to collect multiple `Renderable` items and render them all
/// at once as a single concatenated string.
/// 
/// # Example
/// 
/// ```rust
/// use magik::{TemplateAccumulator, Renderable};
/// 
/// let mut acc = TemplateAccumulator::new();
/// acc.push("Hello, ");
/// acc.push("World!");
/// acc.push(42);
/// 
/// assert_eq!(acc.render(), "Hello, World!42");
/// ```
#[derive(Debug, Clone, Default)]
pub struct TemplateAccumulator {
    templates: Vec<String>,
}

impl TemplateAccumulator {
    /// Creates a new empty template accumulator
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }
    
    /// Creates a new template accumulator with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            templates: Vec::with_capacity(capacity),
        }
    }
    
    /// Adds a renderable item to the accumulator
    pub fn push<T: Renderable>(&mut self, item: T) {
        self.templates.push(item.render());
    }
    
    /// Adds a renderable item to the accumulator and returns self for chaining
    pub fn add<T: Renderable>(mut self, item: T) -> Self {
        self.push(item);
        self
    }
    
    /// Extends the accumulator with an iterator of renderable items
    pub fn extend<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Renderable,
    {
        for item in iter {
            self.push(item);
        }
    }
    
    /// Returns the number of accumulated templates
    pub fn len(&self) -> usize {
        self.templates.len()
    }
    
    /// Returns true if the accumulator is empty
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
    
    /// Clears all accumulated templates
    pub fn clear(&mut self) {
        self.templates.clear();
    }
    
    /// Renders all accumulated templates with a custom separator
    pub fn render_with_separator(&self, separator: &str) -> String {
        self.templates.join(separator)
    }
    
    /// Consumes the accumulator and returns the inner vector of rendered strings
    pub fn into_inner(self) -> Vec<String> {
        self.templates
    }
}

impl Renderable for TemplateAccumulator {
    fn render(self) -> String {
        self.templates.concat()
    }
}

impl<T: Renderable> From<Vec<T>> for TemplateAccumulator {
    fn from(items: Vec<T>) -> Self {
        let mut acc = Self::with_capacity(items.len());
        acc.extend(items);
        acc
    }
}

impl<T: Renderable> FromIterator<T> for TemplateAccumulator {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut acc = Self::new();
        acc.extend(iter);
        acc
    }
}

// Allow adding items with the += operator
impl<T: Renderable> std::ops::AddAssign<T> for TemplateAccumulator {
    fn add_assign(&mut self, item: T) {
        self.push(item);
    }
}

// Allow concatenating two accumulators
impl std::ops::Add for TemplateAccumulator {
    type Output = Self;
    
    fn add(mut self, other: Self) -> Self {
        self.templates.extend(other.templates);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_accumulation() {
        let mut acc = TemplateAccumulator::new();
        acc.push("Hello, ");
        acc.push("World!");
        acc.push(42);
        
        assert_eq!(acc.render(), "Hello, World!42");
    }

    #[test]
    fn test_chaining() {
        let acc = TemplateAccumulator::new()
            .add("Hello, ")
            .add("World!")
            .add(42);
        
        assert_eq!(acc.render(), "Hello, World!42");
    }

    #[test]
    fn test_from_vec() {
        let items = vec!["Hello", "World", "!"];
        let acc = TemplateAccumulator::from(items);
        
        assert_eq!(acc.render(), "HelloWorld!");
    }

    #[test]
    fn test_with_separator() {
        let acc = TemplateAccumulator::new()
            .add("Hello")
            .add("World")
            .add("!");
        
        assert_eq!(acc.render_with_separator(" "), "Hello World !");
        assert_eq!(acc.render_with_separator(", "), "Hello, World, !");
    }

    #[test]
    fn test_operators() {
        let mut acc = TemplateAccumulator::new();
        acc += "Hello";
        acc += " ";
        acc += "World";
        
        let acc2 = TemplateAccumulator::new().add("!");
        let combined = acc + acc2;
        
        assert_eq!(combined.render(), "Hello World!");
    }

    #[test]
    fn test_collect() {
        let items = vec!["a", "b", "c"];
        let acc: TemplateAccumulator = items.into_iter().collect();
        
        assert_eq!(acc.render(), "abc");
    }
}
