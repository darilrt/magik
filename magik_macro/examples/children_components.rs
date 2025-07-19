use magik::{Children, children};
use magik_macro::template;

#[template(source = "<button class=\"{{ props.class }}\">{{ props.text }}</button>")]
pub struct Button<'a> {
    pub class: &'a str,
    pub text: &'a str,
}

#[template(source = "<label>{{ props.text }}</label>")]
pub struct Label<'a> {
    pub text: &'a str,
}

#[template(source = r#"
{{ use crate::Button; }}
{{ use crate::Label; }}
<div class="form-group">
    {{ props.children }}
</div>
"#)]
pub struct Form<'a> {
    pub action: &'a str,
    pub method: &'a str,
    pub children: Children,
}

fn main() {
    let form = Form {
        action: "/submit",
        method: "POST",
        children: children![
            Button {
                class: "btn-submit",
                text: "Submit",
            },
            Label {
                text: "Click to submit the form",
            }
        ],
    };

    println!("{}", form);
}
