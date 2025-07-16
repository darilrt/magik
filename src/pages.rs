use magik_macro::template;

#[template("<h1>Hello, {{ props.name }}!</h1>")]
pub struct InlineGreetingPage<'a> {
    name: &'a str,
}
