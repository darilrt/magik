// trait Renderable with Copy and Clone implementations
pub trait Renderable {
    /// Renders the object to a string.
    fn render(&self) -> String;
}

impl Renderable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl Renderable for &String {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Renderable for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Renderable for i32 {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Renderable for f64 {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Renderable for bool {
    fn render(&self) -> String {
        self.to_string()
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
