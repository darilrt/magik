use magik_macro::template;

// Component for article content
#[template(source = r#"
<article>
    <h2>{{ props.article_title }}</h2>
    <p class="author">By {{ props.author }}</p>
    <p>{{ props.content }}</p>
    <small>Published: {{ props.date }}</small>
</article>
"#)]
pub struct Article<'a> {
    pub article_title: &'a str,
    pub author: &'a str,
    pub content: &'a str,
    pub date: &'a str,
}

// Example of a complete web page using a template
#[template(path = "examples/html_page.tmp")]
pub struct WebPage<'a, T: magik::Renderable> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub content: T,
    pub footer_text: &'a str,
}

fn main() {
    // Example article
    let article = Article {
        article_title: "Getting Started with Rust Templates",
        author: "Jane Developer",
        content: "Rust templates provide a powerful way to generate dynamic content while maintaining type safety. With the magik template system, you can create reusable components that compose well together.",
        date: "2025-01-15",
    };

    // Composing the webpage with the article content
    let webpage = WebPage {
        title: "My Blog",
        subtitle: "Thoughts on programming and technology",
        content: article,
        footer_text: "© 2025 My Blog. All rights reserved.",
    };

    println!("{}", webpage);
}
