use magik_macro::template;

#[template(source = "<h1>Welcome to the Main Page!</h1>")]
pub struct MainPage;

fn main() {
    let main_page = MainPage;
    println!("{}", main_page);
}
