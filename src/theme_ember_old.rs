#![allow(dead_code)]
use amane::Color;

pub const GRAIN_SHADER: &str = "/home/persius/.config/amane/shaders/grain.wgsl";

// Gruvbox dark hard + warm accents
pub const BG_HARD: &str = "#1d2021";
pub const BG0: &str = "#282828";
pub const BG1: &str = "#3c3836";
pub const BG2: &str = "#504945";
pub const BG3: &str = "#665c54";

pub const FG: &str = "#ebdbb2";
pub const FG_DIM: &str = "#d5c4a1";
pub const FG_MUTED: &str = "#a89984";

pub const RED: &str = "#fb4934";
pub const GREEN: &str = "#b8bb26";
pub const YELLOW: &str = "#fabd2f";
pub const BLUE: &str = "#83a598";
pub const PURPLE: &str = "#d3869b";
pub const AQUA: &str = "#8ec07c";
pub const ORANGE: &str = "#fe8019";
pub const GRAY: &str = "#928374";

pub fn color(hex: &str) -> Color {
    Color::from(hex)
}
