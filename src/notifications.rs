use std::collections::HashMap;
use std::time::{Duration, Instant};

use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

const POPUP_SECS: u64 = 3;
const MAX_POPUPS: usize = 3;

pub struct NotifState {
    center_open: bool,
    seen: HashMap<u32, Instant>, // notification id -> first shown
    dismissed_popups: HashMap<u32, bool>,
}

impl Service for NotifState {
    fn new() -> Self {
        Self { center_open: false, seen: HashMap::new(), dismissed_popups: HashMap::new() }
    }
    fn interval() -> Duration { Duration::from_millis(500) }
    fn update(&mut self) -> bool {
        // keep UI ticking while popups are visible
        let list = Notifications::read();
        let now = Instant::now();
        let mut any_popup = false;
        for n in list.list().iter().rev().take(MAX_POPUPS) {
            let id = n.id();
            if *self.dismissed_popups.get(&id).unwrap_or(&false) { continue; }
            let t = self.seen.entry(id).or_insert(now);
            if now.duration_since(*t) < Duration::from_secs(POPUP_SECS) {
                any_popup = true;
            }
        }
        any_popup
    }
}

pub fn toggle_center() {
    let mut s = NotifState::write();
    s.center_open = !s.center_open;
}
pub fn open_center() { NotifState::write().center_open = true; }
pub fn close_center() { NotifState::write().center_open = false; }
pub fn is_center_open() -> bool { NotifState::read().center_open }

fn dismiss_popup(id: u32) {
    NotifState::write().dismissed_popups.insert(id, true);
}

fn active_popups() -> Vec<Notification> {
    let list = Notifications::read();
    let mut s = NotifState::write();
    let now = Instant::now();
    let mut out = Vec::new();
    for n in list.list().iter().rev() {
        let id = n.id();
        if *s.dismissed_popups.get(&id).unwrap_or(&false) { continue; }
        let t = s.seen.entry(id).or_insert(now);
        if now.duration_since(*t) >= Duration::from_secs(POPUP_SECS) { continue; }
        out.push(n.clone());
        if out.len() >= MAX_POPUPS { break; }
    }
    out
}

pub fn popup_view() -> LayerWindow {
    let t = theme::current();
    let popups = active_popups();
    if popups.is_empty() {
        return LayerWindow::new()
            .width(1.0).height(1.0)
            .anchor_vertical(Vertical::Bottom).anchor_horizontal(Horizontal::Right)
            .margin(Margin { top: 0, right: -200, bottom: -200, left: 0 })
            .layer(Layer::Overlay).space(Zone::Ignore)
            .keyboard(Keyboard::None).visible(true).click_through()
            .child(Rectangle::new().width(1.0).height(1.0).fill(Color::TRANSPARENT));
    }

    let n = popups.len();
    let height = (n as f32 * 100.0) + ((n.saturating_sub(1)) as f32 * 10.0);
    let mut cards: Vec<Box<dyn Widget>> = Vec::new();
    for p in popups.into_iter().rev() { // oldest on top of stack
        cards.push(Box::new(popup_card(t, &p)));
    }

    LayerWindow::new()
        .width(360.0).height(height.max(100.0))
        .anchor_vertical(Vertical::Bottom).anchor_horizontal(Horizontal::Right)
        .margin(Margin { top: 0, right: 16, bottom: 16, left: 0 })
        .layer(Layer::Overlay).space(Zone::Ignore)
        .keyboard(Keyboard::None).visible(true)
        .child(
            Column::new(cards)
                .width(Parent).height(Parent).gap(10.0).justify(End)
        )
}

fn popup_card(t: &Theme, n: &Notification) -> Rectangle {
    let id = n.id();
    let id_close = id;
    let id_click = id;
    let summary = n.summary().to_string();
    let body = n.body().to_string();
    let app = n.app_name().to_string();
    let border = match n.urgency() {
        Urgency::Critical => t.palette.red,
        Urgency::Low => t.palette.fg_muted,
        _ => if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border },
    };
    Rectangle::new()
        .width(Parent).height(100.0)
        .radius(t.style.module_radius.max(10.0))
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(18.0).offset(0.0, 8.0).opacity(0.35))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 12.0, right: 12.0, bottom: 12.0, left: 12.0 })
        .clip()
        .cursor(Pointer)
        .on_click(move |_| { Notifications::click(id_click); dismiss_popup(id_click); })
        .child(
            Column::new(children![
                Row::new(children![
                    txt(if app.is_empty() { "Notification".to_string() } else { app.clone() }, t.style.font_size - 2.0, t.palette.fg_muted, false),
                    Rectangle::new().width(220.0).height(1.0).fill(Color::TRANSPARENT),
                    small_btn("×", 24.0, t, move |_| { dismiss_popup(id_close); }),
                ]).width(Parent).gap(4.0).align(Center),
                txt(if summary.is_empty() { "(no title)".into() } else { summary }, t.style.font_size, t.palette.fg, true),
                txt(truncate(&body, 80), t.style.font_size - 2.0, t.palette.fg_muted, false),
            ])
            .width(Parent).gap(4.0)
        )
}

pub fn center_view() -> LayerWindow {
    let t = theme::current();
    let open = NotifState::read().center_open;
    if !open {
        return LayerWindow::new()
            .width(1.0).height(1.0)
            .anchor_vertical(Vertical::Top).anchor_horizontal(Horizontal::Left)
            .margin(Margin { top: -200, right: 0, bottom: 0, left: -200 })
            .layer(Layer::Overlay).space(Zone::Ignore)
            .keyboard(Keyboard::None).visible(true).click_through()
            .child(Rectangle::new().width(1.0).height(1.0).fill(Color::TRANSPARENT));
    }
    let list = Notifications::read();
    let notifs: Vec<Notification> = list.list().iter().rev().cloned().collect();
    drop(list);

    LayerWindow::new()
        .width(420.0).height(560.0)
        .anchor_vertical(Vertical::Top).anchor_horizontal(Horizontal::Left)
        .margin(Margin { top: 60, right: 0, bottom: 0, left: 64 })
        .layer(Layer::Overlay).space(Zone::Ignore)
        .keyboard(Keyboard::Exclusive)
        .on_key(|k| { if k == Key::Escape { close_center(); } })
        .child(center_panel(t, &notifs))
}

pub fn catcher_view() -> LayerWindow {
    let open = NotifState::read().center_open;
    if !open {
        return LayerWindow::new()
            .width(Full).height(Full)
            .anchor_vertical(Vertical::Top).anchor_horizontal(Horizontal::Left)
            .layer(Layer::Top).space(Zone::Ignore)
            .keyboard(Keyboard::None).visible(true).click_through()
            .child(Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT));
    }
    LayerWindow::new()
        .width(Full).height(Full)
        .anchor_vertical(Vertical::Top).anchor_horizontal(Horizontal::Left)
        .layer(Layer::Top).space(Zone::Ignore)
        .keyboard(Keyboard::None).visible(true)
        .child(Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT).on_click(|_| close_center()))
}

fn center_panel(t: &Theme, notifs: &[Notification]) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border };
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    if notifs.is_empty() {
        rows.push(Box::new(
            Rectangle::new().width(Parent).height(80.0).fill(Color::TRANSPARENT).align_child(Center, Center)
                .child(txt("No notifications", t.style.font_size, t.palette.fg_muted, false))
        ));
    } else {
        for n in notifs.iter().take(50) {
            rows.push(Box::new(center_card(t, n)));
        }
    }
    Rectangle::new()
        .width(Parent).height(Parent)
        .radius(t.style.bar_radius.max(14.0))
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(24.0).offset(0.0, 12.0).opacity(0.4))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 16.0, right: 16.0, bottom: 16.0, left: 16.0 })
        .child(
            Column::new(children![
                Row::new(children![
                    txt("󰂚  Notifications", t.style.font_size + 3.0, t.palette.blue, true),
                    Rectangle::new().width(12.0).height(1.0).fill(Color::TRANSPARENT),
                    small_btn("Clear", 60.0, t, |_| Notifications::clear()),
                    small_btn("Close", 60.0, t, |_| close_center()),
                ]).width(Parent).gap(8.0).align(Center),
                ScrollArea::new("notif-center", Column::new(rows).width(Parent).gap(8.0))
                    .width(Parent).height(460.0),
            ])
            .width(Parent).height(Parent).gap(12.0)
        )
}

fn center_card(t: &Theme, n: &Notification) -> Rectangle {
    let id = n.id();
    let summary = n.summary().to_string();
    let body = n.body().to_string();
    let app = n.app_name().to_string();
    let border = match n.urgency() {
        Urgency::Critical => t.palette.red,
        Urgency::Low => t.palette.bg2,
        _ => t.style.module_border,
    };
    Rectangle::new()
        .width(Parent).height(100.0)
        .radius(t.style.module_radius.max(8.0))
        .fill(t.palette.bg0)
        .border(t.style.module_border_width.max(1.0), border)
        .padding(Padding { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 })
        .child(
            Column::new(children![
                Row::new(children![
                    txt(if app.is_empty() { "app".into() } else { app.clone() }, t.style.font_size - 2.0, t.palette.fg_muted, false),
                    Rectangle::new().width(280.0).height(1.0).fill(Color::TRANSPARENT),
                    small_btn("×", 24.0, t, move |_| Notifications::dismiss(id)),
                ]).width(Parent).gap(4.0).align(Center),
                txt(if summary.is_empty() { "(no title)".into() } else { summary }, t.style.font_size - 1.0, t.palette.fg, true),
                txt(truncate(&body, 140), t.style.font_size - 2.0, t.palette.fg_muted, false),
            ]).width(Parent).gap(4.0)
        )
}

fn truncate(s: &str, max: usize) -> String {
    let s = s.replace('\n', " ").trim().to_string();
    if s.chars().count() > max { format!("{}…", s.chars().take(max.saturating_sub(1)).collect::<String>()) } else { s }
}

fn small_btn(label: &str, w: f32, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    Rectangle::new()
        .width(w).height(26.0)
        .radius(t.style.button_radius.max(6.0))
        .fill(t.palette.bg1)
        .border(t.style.module_border_width.max(1.0), t.style.module_border)
        .align_child(Center, Center).cursor(Pointer)
        .on_click(on_click)
        .child(txt(label, t.style.font_size - 2.0, t.palette.fg, false))
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
