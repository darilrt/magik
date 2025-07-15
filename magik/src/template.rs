#[derive(Debug, PartialEq, Clone)]
pub enum TemplateData {
    // Pure HTML data
    Html(String),
    // Key of the template to be replaced
    Code(String),
}
