use magik_macro::template;

#[template(path = "pages/main.tmp")]
pub struct MainPage;

#[template(path = "pages/greeting.tmp")]
pub struct GreetingPage<'a> {
    name: &'a str,
    is_greeting: bool,
}

#[template("<h1>Hello, {{ props.name }}!</h1>")]
pub struct InlineGreetingPage<'a> {
    name: &'a str,
}
