use magik_macro::template;

#[template(path = "pages/main.tmp")]
pub struct MainPage;

#[template(path = "pages/greeting.tmp")]
pub struct GreetingPage {
    name: String,
    is_greeting: bool,
}
