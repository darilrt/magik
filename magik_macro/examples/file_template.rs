use magik_macro::template;

// Example using template from an external file
#[template(path = "magik_macro/examples/email_template.tmp")]
pub struct EmailTemplate<'a> {
    pub name: &'a str,
    pub site_name: &'a str,
    pub is_member: bool,
    pub status: &'a str,
    pub has_premium: bool,
}

fn main() {
    let email = EmailTemplate {
        name: "John Doe",
        site_name: "TechHub",
        is_member: true,
        status: "Active",
        has_premium: false,
    };

    println!("{}", email);

    println!("{}\n", "=".repeat(50));

    let email2 = EmailTemplate {
        name: "Jane Smith",
        site_name: "DevCommunity",
        is_member: false,
        status: "Guest",
        has_premium: true,
    };

    println!("{}", email2);
}
