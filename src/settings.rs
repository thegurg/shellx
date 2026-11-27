use amane::*;

use crate::config::{self, Config};
use crate::theme::{self, GRAIN_SHADER, Theme};

pub fn open() { open_window("settings", view); }
pub fn close() { close_window("settings"); }

pub fn view() -> Window {
    let t = theme::current();
    Window::new()
        .title("Amane Settings")
        .size(560.0, 620.0)
        .resizable(true)
        .on_key(|k| { if k == Key::Escape { close(); } })
        .child(panel(t))
}

fn panel(t: &Theme) -> Rectangle {
    let cfg = config::get();
    Rectangle::new()
        .width(Parent).height(Parent)
        .fill(t.palette.bg_hard)
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 20.0, right: 20.0, bottom: 20.0, left: 20.0 })
        .child(
            Column::new(children![
                header(t),
                section(t, "Theme"),
                row_btn(t, "Cycle theme", &format!("Current: {}", theme::current().name), || { theme::cycle(); }),
                section(t, "Modules (apply on restart)"),
                toggle_row(t, "Bar", cfg.bar_enabled, |v| update_cfg(|c| c.bar_enabled = v)),
                toggle_row(t, "Rail", cfg.rail_enabled, |v| update_cfg(|c| c.rail_enabled = v)),
                toggle_row(t, "Notifications", cfg.notifications_enabled, |v| update_cfg(|c| c.notifications_enabled = v)),
                toggle_row(t, "Monitor", cfg.monitor_enabled, |v| update_cfg(|c| c.monitor_enabled = v)),
                toggle_row(t, "Lock screen", cfg.lock_enabled, |v| update_cfg(|c| c.lock_enabled = v)),
                toggle_row(t, "OSD", cfg.osd_enabled, |v| update_cfg(|c| c.osd_enabled = v)),
                section(t, "Notifications"),
                row_btn(t, "Popup timeout -1s", &format!("{}s", cfg.notification_timeout_secs), || update_cfg(|c| c.notification_timeout_secs = c.notification_timeout_secs.saturating_sub(1).max(1))),
                row_btn(t, "Popup timeout +1s", &format!("{}s", cfg.notification_timeout_secs), || update_cfg(|c| c.notification_timeout_secs = (c.notification_timeout_secs + 1).min(15))),
                section(t, "Config"),
                row_btn(t, "Reload config from disk", "~/.config/amane/config.conf", || { config::reload(); let c = config::get(); let _ = theme::set_by_id(&c.theme); }),
                row_btn(t, "Save current config", "writes config.conf", || { let c = config::get(); let _ = config::save(&c); }),
            ])
            .width(Parent).gap(10.0)
        )
}

fn header(t: &Theme) -> Rectangle {
    Rectangle::new().width(Parent).height(40.0).fill(Color::TRANSPARENT)
        .child(Row::new(children![
            txt("  Settings", t.style.font_size + 5.0, t.palette.orange, true),
            Rectangle::new().width(12.0).height(1.0).fill(Color::TRANSPARENT),
            small_btn("Close", 64.0, t, |_| close()),
        ]).width(Parent).height(Parent).gap(8.0).align(Center))
}

fn section(t: &Theme, label: &str) -> Text {
    txt(label, t.style.font_size + 1.0, t.palette.aqua, true)
}

fn row_btn(t: &Theme, label: &str, value: &str, on_click: impl Fn() + 'static) -> Rectangle {
    Rectangle::new()
        .width(Parent).height(42.0)
        .radius(t.style.module_radius.max(8.0))
        .fill(t.palette.bg0)
        .border(t.style.module_border_width.max(1.0), t.style.module_border)
        .padding(Padding { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
        .align_child(Start, Center)
        .cursor(Pointer)
        .on_click(move |_| on_click())
        .child(Row::new(children![
            txt(label, t.style.font_size, t.palette.fg, false),
            Rectangle::new().width(20.0).height(1.0).fill(Color::TRANSPARENT),
            txt(value, t.style.font_size - 1.0, t.palette.fg_muted, false),
        ]).width(Parent).gap(8.0).align(Center))
}

fn toggle_row(t: &Theme, label: &str, enabled: bool, on_toggle: impl Fn(bool) + 'static) -> Rectangle {
    let value = if enabled { "ON" } else { "OFF" };
    let color = if enabled { t.palette.green } else { t.palette.red };
    row_btn(t, &format!("Toggle {label}"), &format!("{value}"), move || on_toggle(!enabled))
        .border(t.style.module_border_width.max(1.0), color)
}

fn update_cfg(f: impl FnOnce(&mut Config)) {
    let mut s = config::ConfigState::write();
    f(&mut s.cfg);
    let _ = config::save(&s.cfg);
}

fn small_btn(label: &str, w: f32, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    Rectangle::new()
        .width(w).height(30.0)
        .radius(t.style.button_radius.max(6.0))
        .fill(t.palette.bg1)
        .border(t.style.module_border_width.max(1.0), t.style.module_border)
        .align_child(Center, Center).cursor(Pointer)
        .on_click(on_click)
        .child(txt(label, t.style.font_size - 1.0, t.palette.fg, false))
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
