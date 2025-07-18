use magik_macro::template;

#[template(source = "Hello from Magik!")]
pub struct HelloMagikPage;

fn main() {
    println!("{}", HelloMagikPage);
}
