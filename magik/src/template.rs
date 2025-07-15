#[derive(Debug, PartialEq, Clone)]
pub enum TemplateData {
    // Pure string data to be inserted directly into the template
    String(String),
    // Key of the template to be replaced
    Code(String),
}
