#![allow(dead_code)]
use std::time::Duration;

use amane::{Color, Service};

pub const GRAIN_SHADER: &str = "/home/persius/.config/amane/shaders/grain.wgsl";

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg_hard: &'static str,
    pub bg0: &'static str,
    pub bg1: &'static str,
    pub bg2: &'static str,
    pub bg3: &'static str,
    pub fg: &'static str,
    pub fg_dim: &'static str,
    pub fg_muted: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub purple: &'static str,
    pub aqua: &'static str,
    pub orange: &'static str,
    pub gray: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub bar_radius: f32,
    pub module_radius: f32,
    pub button_radius: f32,
    pub bar_bg: &'static str,
    pub bar_border: &'static str,
    pub bar_border_width: f32,
    pub module_bg: &'static str,
    pub module_border: &'static str,
    pub module_border_width: f32,
    pub grain_intensity: f32,
    pub grain_scale: f32,
    pub window_margin_top: i32,
    pub window_margin_x: i32,
    pub bar_height: f32,
    pub module_height: f32,
    pub font_size: f32,
    pub bold: bool,
    pub colorful_modules: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub id: &'static str,
    pub name: &'static str,
    pub palette: Palette,
    pub style: Style,
}

pub const EMBER: Theme = Theme {
    id: "ember",
    name: "Gruvbox Ember",
    palette: Palette {
        bg_hard: "#1d2021",
        bg0: "#282828",
        bg1: "#3c3836",
        bg2: "#504945",
        bg3: "#665c54",
        fg: "#ebdbb2",
        fg_dim: "#d5c4a1",
        fg_muted: "#a89984",
        red: "#fb4934",
        green: "#b8bb26",
        yellow: "#fabd2f",
        blue: "#83a598",
        purple: "#d3869b",
        aqua: "#8ec07c",
        orange: "#fe8019",
        gray: "#928374",
    },
    style: Style {
        bar_radius: 14.0,
        module_radius: 11.0,
        button_radius: 9.0,
        bar_bg: "#1d2021",
        bar_border: "#3c3836",
        bar_border_width: 1.0,
        module_bg: "#32302f",
        module_border: "#504945",
        module_border_width: 1.0,
        grain_intensity: 0.055,
        grain_scale: 1.2,
        window_margin_top: 8,
        window_margin_x: 10,
        bar_height: 44.0,
        module_height: 32.0,
        font_size: 13.0,
        bold: false,
        colorful_modules: false,
    },
};

pub const CHROMA: Theme = Theme {
    id: "chroma",
    name: "Gruvbox Chroma",
    palette: Palette {
        bg_hard: "#1d2021",
        bg0: "#282828",
        bg1: "#3c3836",
        bg2: "#504945",
        bg3: "#665c54",
        fg: "#ebdbb2",
        fg_dim: "#d5c4a1",
        fg_muted: "#a89984",
        red: "#ea6962",
        green: "#a9b665",
        yellow: "#d8a657",
        blue: "#7daea3",
        purple: "#d3869b",
        aqua: "#89b482",
        orange: "#e78a4e",
        gray: "#928374",
    },
    style: Style {
        bar_radius: 0.0,
        module_radius: 0.0,
        button_radius: 0.0,
        bar_bg: "#1d202180",
        bar_border: "#504945",
        bar_border_width: 0.0,
        module_bg: "#1d202180",
        module_border: "#504945",
        module_border_width: 2.0,
        grain_intensity: 0.06,
        grain_scale: 1.5,
        window_margin_top: 10,
        window_margin_x: 10,
        bar_height: 50.0,
        module_height: 38.0,
        font_size: 16.0,
        bold: true,
        colorful_modules: true,
    },
};

pub const EVERFOREST_MIST: Theme = Theme {
    id: "mist",
    name: "Everforest Mist",
    palette: Palette {
        bg_hard: "#2b3339",
        bg0: "#323c41",
        bg1: "#3a454a",
        bg2: "#445055",
        bg3: "#4f5b58",
        fg: "#d3c6aa",
        fg_dim: "#c5c0a0",
        fg_muted: "#9da9a0",
        red: "#e67e80",
        green: "#a7c080",
        yellow: "#dbbc7f",
        blue: "#7fbbb3",
        purple: "#d699b6",
        aqua: "#83c092",
        orange: "#e69875",
        gray: "#859289",
    },
    style: Style {
        bar_radius: 14.0,
        module_radius: 11.0,
        button_radius: 9.0,
        bar_bg: "#2b3339",
        bar_border: "#3a454a",
        bar_border_width: 1.0,
        module_bg: "#323c41",
        module_border: "#445055",
        module_border_width: 1.0,
        grain_intensity: 0.045,
        grain_scale: 1.2,
        window_margin_top: 8,
        window_margin_x: 10,
        bar_height: 44.0,
        module_height: 32.0,
        font_size: 13.0,
        bold: false,
        colorful_modules: false,
    },
};

pub const EVERFOREST_BLOOM: Theme = Theme {
    id: "bloom",
    name: "Everforest Bloom",
    palette: Palette {
        bg_hard: "#2b3339",
        bg0: "#323c41",
        bg1: "#3a454a",
        bg2: "#4f585e",
        bg3: "#56635f",
        fg: "#d3c6aa",
        fg_dim: "#c5c0a0",
        fg_muted: "#9da9a0",
        red: "#e67e80",
        green: "#a7c080",
        yellow: "#dbbc7f",
        blue: "#7fbbb3",
        purple: "#d699b6",
        aqua: "#83c092",
        orange: "#e69875",
        gray: "#859289",
    },
    style: Style {
        bar_radius: 0.0,
        module_radius: 0.0,
        button_radius: 0.0,
        bar_bg: "#2b333980",
        bar_border: "#4f585e",
        bar_border_width: 0.0,
        module_bg: "#2b333980",
        module_border: "#4f585e",
        module_border_width: 2.0,
        grain_intensity: 0.05,
        grain_scale: 1.5,
        window_margin_top: 10,
        window_margin_x: 10,
        bar_height: 50.0,
        module_height: 38.0,
        font_size: 16.0,
        bold: true,
        colorful_modules: true,
    },
};

pub struct ThemeState {
    id: u8, // 0=Ember, 1=Chroma
}

impl Service for ThemeState {
    fn new() -> Self {
        let cfg = crate::config::get();
        let id = id_from_str(&cfg.theme).unwrap_or(1);
        Self { id }
    }

    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

impl ThemeState {
    pub fn current(&self) -> &'static Theme {
        match self.id {
            0 => &EMBER,
            1 => &CHROMA,
            2 => &EVERFOREST_MIST,
            _ => &EVERFOREST_BLOOM,
        }
    }

    pub fn set(&mut self, id: u8) {
        self.id = id;
    }
}

pub fn current() -> &'static Theme {
    ThemeState::read().current()
}

fn id_from_str(id: &str) -> Option<u8> {
    match id.to_ascii_lowercase().as_str() {
        "ember" | "gruvbox-ember" => Some(0),
        "chroma" | "gruvbox-chroma" => Some(1),
        "mist" | "everforest-mist" | "everforest" => Some(2),
        "bloom" | "everforest-bloom" | "everforest-chroma" => Some(3),
        _ => None,
    }
}

pub fn set_by_id(id: &str) -> bool {
    let Some(next) = id_from_str(id) else { return false };
    let mut state = ThemeState::write();
    state.set(next);
    crate::config::set_theme(current_name(next));
    true
}

pub fn cycle() {
    let mut state = ThemeState::write();
    state.id = (state.id + 1) % 4;
    let name = current_name(state.id);
    drop(state);
    crate::config::set_theme(name);
}

pub fn cycle_theme() {
    let mut state = ThemeState::write();
    state.id = match state.id {
        0 => 2, // ember -> mist
        1 => 3, // chroma -> bloom
        2 => 0, // mist -> ember
        _ => 1, // bloom -> chroma
    };
    let name = current_name(state.id);
    drop(state);
    crate::config::set_theme(name);
}

pub fn cycle_variant() {
    let mut state = ThemeState::write();
    state.id = match state.id {
        0 => 1, // ember -> chroma
        1 => 0, // chroma -> ember
        2 => 3, // mist -> bloom
        _ => 2, // bloom -> mist
    };
    let name = current_name(state.id);
    drop(state);
    crate::config::set_theme(name);
}

fn current_name(id: u8) -> &'static str {
    match id { 0 => "ember", 1 => "chroma", 2 => "mist", _ => "bloom" }
}

pub fn all() -> [&'static Theme; 4] {
    [&EMBER, &CHROMA, &EVERFOREST_MIST, &EVERFOREST_BLOOM]
}

pub fn color(hex: &str) -> Color {
    Color::from(hex)
}
