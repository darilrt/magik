use magik_macro::template;

// Example using conditional logic with choose
#[template(source = "Status: {{ props.is_online.choose(\"🟢 Online\", \"🔴 Offline\") }}")]
pub struct UserStatus {
    pub is_online: bool,
}

// Example with multiple conditions
#[template(
    source = "{{ props.is_premium.choose(\"⭐ Premium User\", \"Regular User\") }} - {{ props.has_notifications.choose(\"🔔 New messages\", \"📭 No messages\") }}"
)]
pub struct UserProfile {
    pub is_premium: bool,
    pub has_notifications: bool,
}

// Example of conditional with different types
#[template(
    source = "Weather: {{ props.is_sunny.choose(\"☀️ Sunny\", \"🌧️ Rainy\") }}, Temperature: {{ props.temperature }}°C"
)]
pub struct WeatherInfo {
    pub is_sunny: bool,
    pub temperature: i32,
}

fn main() {
    let status1 = UserStatus { is_online: true };
    let status2 = UserStatus { is_online: false };

    println!("{}", status1);
    println!("{}", status2);

    let profile1 = UserProfile {
        is_premium: true,
        has_notifications: true,
    };
    let profile2 = UserProfile {
        is_premium: false,
        has_notifications: false,
    };

    println!("{}", profile1);
    println!("{}", profile2);

    let weather = WeatherInfo {
        is_sunny: false,
        temperature: 22,
    };
    println!("{}", weather);
}
