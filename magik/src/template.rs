use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateData<'a> {
    // Pure string data to be inserted directly into the template
    String(Cow<'a, str>),
    // Code blocks that will be compiled and executed
    Code(Cow<'a, str>),
}

impl TemplateData<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            TemplateData::String(cow) => cow.as_ref(),
            TemplateData::Code(cow) => cow.as_ref(),
        }
    }
}
