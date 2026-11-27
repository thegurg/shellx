use std::time::{Duration, Instant};

use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind { Volume, Brightness }

pub struct OsdState {
    kind: Kind,
    value: u8,
    muted: bool,
    visible_until: Option<Instant>,
    last_vol: u8,
    last_vol_muted: bool,
    last_bri: u8,
    initialized: bool,
}

impl Service for OsdState {
    fn new() -> Self {
        Self {
            kind: Kind::Volume,
            value: 0,
            muted: false,
            visible_until: None,
            last_vol: 0,
            last_vol_muted: false,
            last_bri: 0,
            initialized: false,
        }
    }
    fn interval() -> Duration { Duration::from_millis(200) }
    fn update(&mut self) -> bool {
        let audio = Audio::read();
        let vol = audio.volume();
        let vol_muted = audio.muted();
        drop(audio);
        let bri = Brightness::read().percent();

        if !self.initialized {
            self.last_vol = vol;
            self.last_vol_muted = vol_muted;
            self.last_bri = bri;
            self.initialized = true;
            return false;
        }

        let mut changed = false;
        let now = Instant::now();
        if vol != self.last_vol || vol_muted != self.last_vol_muted {
            self.kind = Kind::Volume;
            self.value = vol;
            self.muted = vol_muted;
            self.visible_until = Some(now + Duration::from_millis(1500));
            self.last_vol = vol;
            self.last_vol_muted = vol_muted;
            changed = true;
        }
        if bri != self.last_bri {
            self.kind = Kind::Brightness;
            self.value = bri;
            self.muted = false;
            self.visible_until = Some(now + Duration::from_millis(1500));
            self.last_bri = bri;
            changed = true;
        }
        if self.visible_until.map(|t| now >= t).unwrap_or(false) {
            self.visible_until = None;
            changed = true;
        }
        changed || self.visible_until.is_some()
    }
}

pub fn view() -> LayerWindow {
    let t = theme::current();
    let s = OsdState::read();
    let visible = s.visible_until.map(|t| Instant::now() < t).unwrap_or(false);
    let kind = s.kind;
    let value = s.value;
    let muted = s.muted;
    drop(s);

    if !visible {
        return LayerWindow::new()
            .width(1.0).height(1.0)
            .anchor_vertical(Vertical::Bottom).anchor_horizontal(Horizontal::Middle)
            .margin(Margin { top: 0, right: 0, bottom: -200, left: 0 })
            .layer(Layer::Overlay).space(Zone::Ignore)
            .keyboard(Keyboard::None).visible(true).click_through()
            .child(Rectangle::new().width(1.0).height(1.0).fill(Color::TRANSPARENT));
    }

    let (icon, color, label) = match kind {
        Kind::Volume => {
            let icon = if muted || value == 0 { "󰝟" } else if value < 35 { "󰕿" } else if value < 70 { "󰖀" } else { "󰕾" };
            let label = if muted { "Muted".to_string() } else { format!("{}%", value) };
            (icon, t.palette.blue, label)
        }
        Kind::Brightness => ("󰃠", t.palette.yellow, format!("{}%", value)),
    };

    LayerWindow::new()
        .width(320.0).height(56.0)
        .anchor_vertical(Vertical::Bottom).anchor_horizontal(Horizontal::Middle)
        .margin(Margin { top: 0, right: 0, bottom: 90, left: 0 })
        .layer(Layer::Overlay).space(Zone::Ignore)
        .keyboard(Keyboard::None).visible(true).click_through()
        .child(
            Rectangle::new()
                .width(Parent).height(Parent)
                .radius(t.style.bar_radius.max(12.0))
                .fill(t.style.bar_bg)
                .border(t.style.module_border_width.max(1.0), if t.style.colorful_modules { color } else { t.style.bar_border })
                .shadow(Shadow::drop("#000000").blur(20.0).offset(0.0, 8.0).opacity(0.35))
                .shader(GRAIN_SHADER)
                .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
                .padding(Padding { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 })
                .child(
                    Row::new(children![
                        txt(icon, t.style.font_size + 4.0, color, true),
                        bar(t, value, color),
                        txt(label, t.style.font_size, t.palette.fg, true),
                    ]).width(Parent).height(Parent).gap(12.0).align(Center)
                )
        )
}

fn bar(t: &Theme, value: u8, color: &str) -> Rectangle {
    let w = (value.min(100) as f32 / 100.0) * 180.0;
    Rectangle::new()
        .width(180.0).height(10.0)
        .radius(5.0).fill(t.palette.bg2).clip()
        .child(Rectangle::new().width(w.max(2.0)).height(Parent).radius(5.0).fill(color))
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
