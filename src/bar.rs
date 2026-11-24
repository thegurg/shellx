use amane::*;

use crate::active_window::ActiveWindow;
use crate::clock::Clock;
use crate::theme::{self, GRAIN_SHADER, Theme};

pub fn view(monitor: &Monitor) -> LayerWindow {
    let t = theme::current();
    LayerWindow::new()
        .width(Full)
        .height(t.style.bar_height)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .margin(Margin {
            top: t.style.window_margin_top,
            right: t.style.window_margin_x,
            bottom: 0,
            left: t.style.window_margin_x,
        })
        .child(bar_shell(monitor, t))
}

fn bar_shell(monitor: &Monitor, t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .radius(t.style.bar_radius)
        .fill(t.style.bar_bg)
        .border(t.style.bar_border_width, t.style.bar_border)
        .shadow(Shadow::drop("#000000").blur(16.0).offset(0.0, 4.0).opacity(0.30))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 5.0, right: 10.0, bottom: 5.0, left: 10.0 })
        .child(
            Row::new(children![left(monitor, t), center(t), right(t)])
                .width(Parent)
                .height(Parent)
                .align(Center),
        )
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}

fn left(monitor: &Monitor, t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(Start, Center)
        .child(
            Row::new(children![
                rail_button(t),
                workspaces(monitor, t),
                window_title(t),
            ])
            .gap(10.0)
            .align(Center),
        )
}

fn center(t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(Center, Center)
        .child(clock_widget(t))
}

fn right(t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(End, Center)
        .child(
            Row::new(children![
                audio_widget(t),
                network_widget(t),
                battery_widget(t),
                media_indicator(t),
                control_center_button(t),
                power_buttons(t),
            ])
            .gap(8.0)
            .align(Center),
        )
}



fn rail_button(t: &Theme) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.aqua } else { t.style.module_border };
    clickable_module(44.0, "󰍜".to_string(), t.palette.aqua, border, t, |_| crate::rail::toggle())
}

fn workspaces(monitor: &Monitor, t: &Theme) -> Rectangle {
    let ws = Workspaces::read();
    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();
    for workspace in ws.list() {
        if workspace.output() != Some(monitor.name.as_str()) { continue; }
        buttons.push(Box::new(workspace_button(workspace, t)));
    }
    if buttons.is_empty() {
        buttons.push(Box::new(Rectangle::new().width(28.0).height(28.0).align_child(Center, Center)
            .child(txt("•", t.style.font_size, t.palette.fg_muted, t.style.bold))));
    }
    let count = buttons.len().max(1) as f32;
    let width = count * 30.0 + (count - 1.0).max(0.0) * 6.0 + 20.0;
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.module_border };
    Rectangle::new()
        .width(width)
        .height(t.style.module_height)
        .radius(t.style.module_radius)
        .fill(t.style.module_bg)
        .border(t.style.module_border_width, border)
        .padding(Padding { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .align_child(Center, Center)
        .child(Row::new(buttons).gap(6.0).align(Center))
}

fn workspace_button(workspace: &Workspace, t: &Theme) -> Rectangle {
    let id = workspace.id();
    let focused = workspace.focused();
    let urgent = workspace.urgent();
    let active = workspace.active();
    let (bg, fg, border) = if focused {
        (t.palette.orange, t.palette.bg_hard, t.palette.yellow)
    } else if urgent {
        (t.palette.red, t.palette.bg_hard, t.palette.red)
    } else if active {
        (t.palette.bg1, t.palette.fg, t.palette.bg2)
    } else {
        (t.palette.bg0, t.palette.fg_muted, t.palette.bg1)
    };
    let label = if t.style.colorful_modules { workspace.index().to_string() }
        else if focused { "●".to_string() } else { "○".to_string() };
    Rectangle::new()
        .width(30.0).height(30.0)
        .radius(t.style.button_radius)
        .fill(bg)
        .border(t.style.module_border_width.max(1.0), border)
        .align_child(Center, Center)
        .cursor(Pointer)
        .on_click(move |_| Workspaces::focus(id))
        .child(txt(label, t.style.font_size - 1.0, fg, t.style.bold))
}

fn window_title(t: &Theme) -> Rectangle {
    let win = ActiveWindow::read();
    let title = win.title();
    let display = if title.chars().count() > 36 {
        let mut s: String = title.chars().take(36).collect(); s.push('…'); s
    } else if title.is_empty() { String::from("Desktop") } else { title.to_string() };
    let border = if t.style.colorful_modules { t.palette.blue } else { t.palette.bg3 };
    Rectangle::new()
        .width(340.0)
        .height(t.style.module_height)
        .radius(t.style.module_radius)
        .fill(t.style.module_bg)
        .border(t.style.module_border_width, border)
        .padding(Padding { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
        .align_child(Start, Center)
        .child(txt(display, t.style.font_size, t.palette.fg, t.style.bold).elide())
}

fn clock_widget(t: &Theme) -> Rectangle {
    let clock = Clock::read();
    let border = if t.style.colorful_modules { t.palette.red } else { t.style.module_border };
    Rectangle::new()
        .width(280.0)
        .height(t.style.module_height)
        .radius(t.style.module_radius)
        .fill(t.style.module_bg)
        .border(t.style.module_border_width, border)
        .padding(Padding { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
        .align_child(Center, Center)
        .child(
            Row::new(children![
                txt(clock.date(), t.style.font_size - 2.0, t.palette.fg_muted, t.style.bold),
                Rectangle::new().width(2.0).height(18.0).radius(1.0).fill(t.palette.bg3),
                txt(clock.time(), t.style.font_size, t.palette.yellow, true),
            ])
            .gap(10.0)
            .align(Center),
        )
}

fn audio_widget(t: &Theme) -> Rectangle {
    let audio = Audio::read();
    let vol = audio.volume();
    let muted = audio.muted();
    let icon = if muted { "󰖁" } else if vol <= 33 { "󰕿" } else if vol <= 66 { "󰖀" } else { "󰕾" };
    let label = if muted { format!("{icon}  muted") } else { format!("{icon}  {vol}%") };
    let fg = if muted { t.palette.fg_muted } else { t.palette.blue };
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.module_border };
    clickable_module(120.0, label, fg, border, t, |_| crate::control_center::toggle())
}

fn network_widget(t: &Theme) -> Rectangle {
    let net = Network::read();
    let (label, fg) = if !net.connected() {
        ("󰤭  off".to_string(), t.palette.fg_muted)
    } else {
        let ssid = net.ssid();
        let name = if ssid.is_empty() { "net".to_string() } else {
            if ssid.chars().count() > 10 { let mut s: String = ssid.chars().take(10).collect(); s.push('…'); s } else { ssid.to_string() }
        };
        let strength = net.strength();
        let icon = if strength >= 80 { "󰤨" } else if strength >= 60 { "󰤥" } else if strength >= 40 { "󰤢" } else if strength >= 20 { "󰤟" } else { "󰤯" };
        (format!("{icon}  {name}"), t.palette.yellow)
    };
    let border = if t.style.colorful_modules { t.palette.yellow } else { t.style.module_border };
    clickable_module(136.0, label, fg, border, t, |_| crate::control_center::toggle())
}


fn battery_widget(t: &Theme) -> Rectangle {
    let bat = Battery::read();
    if !bat.present() {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    }
    let p = bat.percent();
    let charging = bat.charging();
    let full = bat.full();
    let icon = if charging { "󱐋" } else if p >= 90 { "󰁹" } else if p >= 70 { "󰂁" } else if p >= 50 { "󰁿" } else if p >= 30 { "󰁽" } else if p >= 15 { "󰁻" } else { "󰂎" };
    let fg = if charging || full { t.palette.green } else if p <= 15 { t.palette.red } else if p <= 30 { t.palette.orange } else { t.palette.fg };
    let border = if t.style.colorful_modules { t.palette.purple } else { t.style.module_border };
    clickable_module(104.0, format!("{p}% {icon}"), fg, border, t, |_| crate::control_center::toggle())
}


fn media_indicator(t: &Theme) -> Rectangle {
    let m = Media::read();
    let Some(p) = m.active() else {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    };
    let icon = if p.playing() { "󰎈" } else { "󰏤" };
    let border = if t.style.colorful_modules { t.palette.purple } else { t.style.module_border };
    clickable_module(48.0, icon.to_string(), if p.playing() { t.palette.purple } else { t.palette.fg_muted }, border, t, |_| crate::control_center::toggle())
}

fn control_center_button(t: &Theme) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.module_border };
    clickable_module(48.0, "󰒓".to_string(), t.palette.fg, border, t, |_| crate::control_center::toggle())
}

fn power_buttons(t: &Theme) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.red } else { t.style.module_border };
    Rectangle::new()
        .width(84.0)
        .height(t.style.module_height)
        .radius(t.style.module_radius)
        .fill(t.style.module_bg)
        .border(t.style.module_border_width, border)
        .padding(Padding { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
        .align_child(Center, Center)
        .child(
            Row::new(children![
                mini_button("↻", t.palette.orange, t, |_| { amane::spawn("systemctl reboot") }),
                mini_button("", t.palette.red, t, |_| { amane::spawn("systemctl poweroff") }),
            ])
            .gap(6.0)
            .align(Center),
        )
}


fn clickable_module(width: f32, label: String, fg: &str, border: &str, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(t.style.module_height)
        .radius(t.style.module_radius)
        .fill(t.style.module_bg)
        .border(t.style.module_border_width, border)
        .padding(Padding { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .align_child(Center, Center)
        .cursor(Pointer)
        .on_click(on_click)
        .child(txt(label, t.style.font_size, fg, t.style.bold))
}

fn mini_button(label: &str, fg: &str, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    Rectangle::new()
        .width(30.0).height(30.0)
        .radius(t.style.button_radius)
        .fill(t.palette.bg1)
        .border(t.style.module_border_width.max(1.0), t.palette.bg2)
        .align_child(Center, Center)
        .cursor(Pointer)
        .on_click(on_click)
        .child(txt(label, t.style.font_size, fg, true))
}
