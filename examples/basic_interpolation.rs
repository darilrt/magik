use magik_macro::template;

// Basic example of variable interpolation
#[template(source = "Hello, {{ props.name }}! You are {{ props.age }} years old.")]
pub struct PersonGreeting<'a> {
    pub name: &'a str,
    pub age: i32,
}

// Example with multiple data types
#[template(source = "Price: ${{ props.price }}, In stock: {{ props.in_stock }}")]
pub struct ProductInfo {
    pub price: f64,
    pub in_stock: bool,
}

fn main() {
    let person = PersonGreeting {
        name: "Alice",
        age: 30,
    };
    println!("{}", person);

    let product = ProductInfo {
        price: 29.99,
        in_stock: true,
    };
    println!("{}", product);
}
