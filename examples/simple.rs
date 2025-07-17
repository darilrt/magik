use magik_macro::template;

#[template(source = "Welcome to the Main Page!")]
pub struct MainPage;

fn main() {
    println!("{}", MainPage);
}
