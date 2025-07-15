mod pages;

use magik::Renderable;
use pages::MainPage;

fn main() {
    let a = MainPage {};

    println!("{}", a.render());
}
