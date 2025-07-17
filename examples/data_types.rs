use magik_macro::template;

// Example with multiple data types
#[template(source = "ID: {{ props.id }}, Score: {{ props.score }}, Active: {{ props.is_active }}")]
pub struct GameStats {
    pub id: i32,
    pub score: f64,
    pub is_active: bool,
}

// Example with strings and numbers
#[template(
    source = "Player: {{ props.username }} (Level {{ props.level }}) - {{ props.online.choose(\"Online\", \"Offline\") }}"
)]
pub struct PlayerInfo<'a> {
    pub username: &'a str,
    pub level: i32,
    pub online: bool,
}

// Example with more complex formatting
#[template(source = r#"
=== GAME REPORT ===
Player: {{ props.player_name }}
Current Level: {{ props.current_level }}
Experience Points: {{ props.exp_points }}
Gold: {{ props.gold_amount }}
Status: {{ props.is_playing.choose("🎮 Currently Playing", "💤 Offline") }}
Premium: {{ props.has_premium.choose("✨ Premium Member", "👤 Free Player") }}
==================="#)]
pub struct GameReport<'a> {
    pub player_name: &'a str,
    pub current_level: i32,
    pub exp_points: i32,
    pub gold_amount: f64,
    pub is_playing: bool,
    pub has_premium: bool,
}

fn main() {
    let stats = GameStats {
        id: 12345,
        score: 98.5,
        is_active: true,
    };
    println!("{}", stats);

    let player = PlayerInfo {
        username: "DragonSlayer99",
        level: 42,
        online: true,
    };
    println!("{}", player);

    let report = GameReport {
        player_name: "Alice Wonderland",
        current_level: 67,
        exp_points: 125430,
        gold_amount: 1234.56,
        is_playing: false,
        has_premium: true,
    };
    println!("{}", report);
}
