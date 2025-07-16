use magik::Renderable;
use magik_macro::template;

#[template(source = "<h1>Welcome to the Main Page!</h1>")]
pub struct SimpleBody;

#[template(
    r#"
<!DOCTYPE html>
<html lang="en">
    <head>
    </head>
    <body>
        {{
            let a = "KK Code";
            props.body
        }}
        {{ props.title }}
    </body>
</html>
"#
)]
pub struct Layout<T: magik::Renderable> {
    pub body: T,
    pub title: String,
}
