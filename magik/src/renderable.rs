use crate::Error;

macro_rules! impl_renderable_with_to_string {
    ($type:ty) => {
        impl Renderable for $type {
            fn render(&self) -> String {
                self.to_string()
            }
        }
    };
    ($type:ty, $($rest:ty),+) => {
        impl_renderable_with_to_string!($type);
        impl_renderable_with_to_string!($($rest),+);
    };
}

// Implementations for primitive types and common types
impl_renderable_with_to_string!(
    String, &String, &str, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64, bool, char
);

/// Trait for types that can be rendered to a string.
/// This trait is used when the rendering logic is simple and does not require error handling.
pub trait Renderable {
    /// Renders the object to a string.
    fn render(&self) -> String;
}

/// Trait for types that can be rendered with error handling.
/// This is useful for templates that may fail to render.
/// This trait allows for more complex rendering logic that can handle errors gracefully.
pub trait TryRenderable {
    /// Attempts to render the object, returning a Result.
    fn try_render(&self) -> Result<String, Error>;
}

impl<T: Renderable + 'static> From<T> for Box<dyn Renderable> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

impl<T: Renderable> Renderable for Vec<T> {
    fn render(&self) -> String {
        self.iter()
            .map(|item| item.render())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Renderable for Vec<Box<dyn Renderable>> {
    fn render(&self) -> String {
        self.iter()
            .map(|item| item.render())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T: Renderable> Renderable for &[T] {
    fn render(&self) -> String {
        self.iter()
            .map(|item| item.render().clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T: Renderable> Renderable for Option<T> {
    fn render(&self) -> String {
        match self {
            Some(value) => value.render(),
            None => String::new(),
        }
    }
}

impl Renderable for () {
    fn render(&self) -> String {
        String::new()
    }
}
