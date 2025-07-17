use magik_macro::template;

#[template(
    r#"
{{ use crate::pages::Layout; }}
{{ use crate::pages::Button; }}
{{
    Layout { 
        title: "Main Page",
        body: vec![
            "<p>Welcome to the main page!</p>".boxed(),
            Button { label: "Click Me" }.boxed(),
        ]
    }
}}
"#
)]
pub struct MainPage {}

#[template(
    r#"
<!DOCTYPE html>
<html lang="en">
    <head>
    </head>
    <body>
        <h1>{{ props.title }}</h1>
        {{ props.body }}
    </body>
</html>
"#
)]
pub struct Layout<'a, T: magik::Renderable> {
    pub title: &'a str,
    pub body: T,
}

#[template("<button>{{ props.label }}</button>")]
pub struct Button<'a> {
    pub label: &'a str,
}
