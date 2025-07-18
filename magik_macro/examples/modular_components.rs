use magik_macro::template;

// Simple component for list item
#[template(source = "<li>{{ props.text }}</li>")]
pub struct ListItem<'a> {
    pub text: &'a str,
}

// Componente for section header
#[template(source = "<h2>{{ props.title }}</h2>")]
pub struct SectionHeader<'a> {
    pub title: &'a str,
}

// Componente for paragraph with conditional styling
#[template(
    source = r#"<p class="{{ props.is_important.choose("important", "normal") }}">{{ props.content }}</p>"#
)]
pub struct Paragraph<'a> {
    pub content: &'a str,
    pub is_important: bool,
}

// Dashboard that composes multiple elements
#[template(source = r#"
{{ use crate::SectionHeader; }}
{{ use crate::Paragraph; }}
{{ use crate::ListItem; }}
<div class="dashboard">
    {{ SectionHeader { title: "System Status" } }}
    {{ Paragraph { content: props.status_message, is_important: props.has_alerts } }}
    
    {{ SectionHeader { title: "Quick Actions" } }}
    <ul>
        {{ ListItem { text: "View Logs" } }}
        {{ ListItem { text: "Run Diagnostics" } }}
        {{ ListItem { text: "Backup Data" } }}
    </ul>
    
    {{ SectionHeader { title: "Statistics" } }}
    {{ Paragraph { content: props.stats_summary, is_important: false } }}
</div>
"#)]
pub struct Dashboard<'a> {
    pub status_message: &'a str,
    pub has_alerts: bool,
    pub stats_summary: &'a str,
}

fn main() {
    let dashboard = Dashboard {
        status_message: "All systems operational. No critical issues detected.",
        has_alerts: false,
        stats_summary: "Total users: 1,234 | Active sessions: 89 | Server uptime: 99.9%",
    };

    println!("{}", dashboard);

    println!("\n{}\n", "-".repeat(60));

    let alert_dashboard = Dashboard {
        status_message: "WARNING: High memory usage detected on server cluster!",
        has_alerts: true,
        stats_summary: "Memory usage: 89% | CPU usage: 76% | Disk space: 23% remaining",
    };

    println!("{}", alert_dashboard);
}
