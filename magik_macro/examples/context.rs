use magik_macro::template;

// Example usage of the magik_macro template with a context config
#[template(source = "<div>{{ data.name }}</div>", context = "data")]
struct MyStruct {
    name: String,
}

// This template will generate a function that renders the HTML with the provided context.
#[template(
    source = "<h1>Hello, {{ foo.name }}!</h1> {{ foo.test }}",
    context = "foo"
)]
struct MyTemplate {
    name: String,
    test: String,
}

// If the context is not provided it will default to "props"
#[template(source = "<p>{{ props.text }}</p>")]
struct DefaultContextTemplate {
    text: String,
}

fn main() {
    let my_struct = MyStruct {
        name: "World".to_string(),
    };
    println!("{}", my_struct);

    let my_template = MyTemplate {
        name: "Alice".to_string(),
        test: "This is a test".to_string(),
    };
    println!("{}", my_template);

    let default_context_template = DefaultContextTemplate {
        text: "Hello, default context!".to_string(),
    };
    println!("{}", default_context_template);
}
