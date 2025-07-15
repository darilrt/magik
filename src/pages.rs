use magik_macro::template;

#[template(path = "pages/main.tmp")]
pub struct MainPage;

#[template(path = "pages/greeting.tmp")]
pub struct GreetingPage {
    name: &'static str,
    is_greeting: bool,
}
