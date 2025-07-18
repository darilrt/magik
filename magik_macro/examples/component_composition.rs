use magik_macro::template;

// Basic component for a button
#[template(source = "<button class=\"{{ props.class }}\">{{ props.text }}</button>")]
pub struct Button<'a> {
    pub class: &'a str,
    pub text: &'a str,
}

// Component for a link
#[template(source = "<a href=\"{{ props.url }}\">{{ props.label }}</a>")]
pub struct Link<'a> {
    pub url: &'a str,
    pub label: &'a str,
}

// Component that composes other components
#[template(source = r#"
{{ use crate::Button; }}
{{ use crate::Link; }}
<div class="card">
    <h3>{{ props.title }}</h3>
    <p>{{ props.description }}</p>
    <div class="actions">
        {{ Button { class: "btn-primary", text: "Learn More" } }}
        {{ Link { url: props.link_url, label: "External Link" } }}
    </div>
</div>
"#)]
pub struct Card<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub link_url: &'a str,
}

// Main layout that uses the components
#[template(source = r#"
<!DOCTYPE html>
<html>
<head>
    <title>{{ props.page_title }}</title>
</head>
<body>
    <header>
        <h1>{{ props.site_name }}</h1>
    </header>
    <main>
        {{ props.content }}
    </main>
</body>
</html>
"#)]
pub struct Layout<'a, T: magik::Renderable> {
    pub page_title: &'a str,
    pub site_name: &'a str,
    pub content: T,
}

fn main() {
    let card = Card {
        title: "Amazing Product",
        description: "This is an incredible product that will change your life!",
        link_url: "https://example.com",
    };

    let page = Layout {
        page_title: "Product Showcase",
        site_name: "My Awesome Site",
        content: card,
    };

    println!("{}", page);
}
