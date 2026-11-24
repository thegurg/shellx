use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

pub struct ControlCenterState {
    open: bool,
}

impl Service for ControlCenterState {
    fn new() -> Self { Self { open: false } }
    fn interval() -> std::time::Duration { std::time::Duration::from_secs(3600) }
}

pub fn toggle() { let mut s = ControlCenterState::write(); s.open = !s.open; }
pub fn open() { ControlCenterState::write().open = true; }
pub fn close() { ControlCenterState::write().open = false; }
pub fn is_open() -> bool { ControlCenterState::read().open }

pub fn catcher_view() -> LayerWindow {
    let open = ControlCenterState::read().open;
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
        .layer(Layer::Top)
        .space(Zone::Ignore)
        .keyboard(Keyboard::None)
        .visible(true)
        .child(
            Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT)
                .on_click(|_| close())
        )
}

pub fn view() -> LayerWindow {
    let open = ControlCenterState::read().open;
    if !open {
        return LayerWindow::new()
            .width(380.0).height(680.0)
            .anchor_vertical(Vertical::Top).anchor_horizontal(Horizontal::Right)
            .margin(Margin { top: 64, right: 14, bottom: 0, left: 0 })
            .layer(Layer::Overlay).space(Zone::Ignore)
            .keyboard(Keyboard::None).visible(true).click_through()
            .child(Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT));
    }
    let t = theme::current();
    LayerWindow::new()
        .width(380.0)
        .height(680.0)
        .anchor_vertical(Vertical::Top)
        .anchor_horizontal(Horizontal::Right)
        .margin(Margin { top: 64, right: 14, bottom: 0, left: 0 })
        .layer(Layer::Overlay)
        .space(Zone::Ignore)
        .keyboard(Keyboard::Exclusive)
        .visible(true)
        .on_key(|k| { if k == Key::Escape { close(); } })
        .child(panel(t))
}

fn panel(t: &Theme) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border };
    Rectangle::new()
        .width(Parent).height(Parent)
        .radius(t.style.bar_radius.max(12.0))
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(24.0).offset(0.0, 10.0).opacity(0.40))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 16.0, right: 16.0, bottom: 16.0, left: 16.0 })
        .child(
            Column::new(children![
                header(t),
                ScrollArea::new("control-center-scroll",
                    Column::new(children![
                        audio_section(t),
                        brightness_section(t),
                        network_section(t),
                        bluetooth_section(t),
                        media_section(t),
                        battery_section(t),
                        power_section(t),
                    ])
                    .width(Parent).gap(12.0)
                ).width(Parent).height(Parent),
            ])
            .width(Parent).height(Parent).gap(12.0),
        )
}

fn header(t: &Theme) -> Rectangle {
    Rectangle::new().width(Parent).height(32.0).align_child(Start, Center).child(
        Row::new(children![
            txt("Control Center", t.style.font_size + 3.0, t.palette.fg, true),
            Rectangle::new().width(Parent).height(1.0),
            Rectangle::new().width(28.0).height(28.0).radius(t.style.button_radius).fill(t.palette.bg1)
                .align_child(Center, Center).cursor(Pointer).on_click(|_| close())
                .child(txt("✕", t.style.font_size, t.palette.red, true)),
        ]).width(Parent).height(Parent).align(Center),
    )
}

fn section(t: &Theme, title: &str, accent: &str, height: f32, child: impl Widget + 'static) -> Rectangle {
    let border = if t.style.colorful_modules { accent } else { t.style.module_border };
    Rectangle::new().width(Parent).height(height).radius(t.style.module_radius)
        .fill(t.palette.bg0).border(t.style.module_border_width.max(1.0), border)
        .padding(Padding { top: 12.0, right: 12.0, bottom: 12.0, left: 12.0 })
        .child(Column::new(children![
            txt(title, t.style.font_size - 1.0, accent, true),
            child,
        ]).width(Parent).gap(10.0))
}

fn slider(t: &Theme, width: f32, value: u8, accent: &str, on_set: impl Fn(u8) + Clone + 'static) -> Rectangle {
    let pct = (value.min(100) as f32) / 100.0;
    let filled = (width * pct).max(6.0);
    let set1 = on_set.clone();
    let set2 = on_set;
    Rectangle::new().width(width).height(14.0).radius(7.0).fill(t.palette.bg2)
        .cursor(Pointer)
        .on_scroll(move |s| {
            let cur = value as i32;
            let d = if s.y < 0.0 { 5 } else if s.y > 0.0 { -5 } else { 0 };
            if d != 0 { set1((cur + d).clamp(0, 100) as u8); }
        })
        .on_drag(move |p| {
            let pct = (p.x / width).clamp(0.0, 1.0);
            set2((pct * 100.0).round() as u8);
        })
        .child(Rectangle::new().width(filled).height(14.0).radius(7.0).fill(accent))
}

fn audio_section(t: &Theme) -> Rectangle {
    let a = Audio::read();
    let vol = a.volume();
    let muted = a.muted();
    let label = if muted { format!("󰖁  Muted") } else { format!("󰕾  Volume  {vol}%") };
    section(t, "Audio", t.palette.blue, 118.0,
        Column::new(children![
            Row::new(children![
                txt(label, t.style.font_size - 1.0, t.palette.fg, t.style.bold),
                Rectangle::new().width(Parent).height(1.0),
                small_btn(if muted { "Unmute" } else { "Mute" }, 84.0, t, |_| Audio::toggle_mute()),
            ]).width(Parent).align(Center),
            slider(t, 320.0, vol, t.palette.blue, |v| {
                Audio::set_volume(v);
            }),
        ]).width(Parent).gap(8.0)
    )
}

fn brightness_section(t: &Theme) -> Rectangle {
    let b = Brightness::read();
    if !b.present() {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    }
    let p = b.percent();
    section(t, "Brightness", t.palette.yellow, 96.0,
        Column::new(children![
            txt(format!("󰃠  {p}%"), t.style.font_size - 1.0, t.palette.fg, t.style.bold),
            slider(t, 320.0, p, t.palette.yellow, |v| {
                Brightness::set(v);
            }),
        ]).width(Parent).gap(8.0)
    )
}

fn network_section(t: &Theme) -> Rectangle {
    let n = Network::read();
    let (label, fg) = if !n.connected() {
        ("Disconnected".to_string(), t.palette.fg_muted)
    } else {
        let ssid = n.ssid();
        let name = if ssid.is_empty() { "Connected".to_string() } else { ssid.to_string() };
        (format!("󰤨  {name} ({}%)", n.strength()), t.palette.fg)
    };
    section(t, "Network", t.palette.aqua, 86.0,
        Row::new(children![
            txt(label, t.style.font_size - 1.0, fg, t.style.bold),
            Rectangle::new().width(Parent).height(1.0),
            small_btn("nmtui", 84.0, t, |_| amane::spawn("kitty -e nmtui")),
        ]).width(Parent).align(Center)
    )
}

fn bluetooth_section(t: &Theme) -> Rectangle {
    let bt = Bluetooth::read();
    if !bt.available() {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    }
    let powered = bt.powered();
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    rows.push(Box::new(Row::new(children![
        txt(if powered { "󰂯  On" } else { "󰂲  Off" }, t.style.font_size - 1.0, t.palette.fg, t.style.bold),
        Rectangle::new().width(Parent).height(1.0),
        small_btn(if powered { "Disable" } else { "Enable" }, 84.0, t, move |_| Bluetooth::set_powered(!powered)),
    ]).width(Parent).align(Center)));

    if powered {
        for dev in bt.devices().iter().take(3) {
            let name = dev.name().to_string();
            let connected = dev.connected();
            let path = dev.path().to_string();
            let label = if connected { format!("󰂱 {name}") } else { format!("󰂯 {name}") };
            let btn = if connected { "Disconnect" } else { "Connect" };
            let p2 = path.clone();
            rows.push(Box::new(Row::new(children![
                txt(label, t.style.font_size - 2.0, if connected { t.palette.blue } else { t.palette.fg_muted }, false),
                Rectangle::new().width(Parent).height(1.0),
                small_btn(btn, 92.0, t, move |_| if connected { Bluetooth::disconnect(&p2) } else { Bluetooth::connect(&p2) }),
            ]).width(Parent).align(Center)));
        }
    }

    let h = 86.0 + if powered { (bt.devices().len().min(3) as f32) * 38.0 } else { 0.0 };
    section(t, "Bluetooth", t.palette.blue, h,
        Column::new(rows).width(Parent).gap(8.0)
    )
}

fn media_section(t: &Theme) -> Rectangle {
    let m = Media::read();
    let Some(p) = m.active() else {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    };
    let title = if p.title().is_empty() { "Unknown" } else { p.title() };
    let artist = p.artist();
    let playing = p.playing();
    section(t, "Media", t.palette.purple, 132.0,
        Column::new(children![
            txt(title.to_string(), t.style.font_size - 1.0, t.palette.fg, true).elide(),
            txt(artist.to_string(), t.style.font_size - 3.0, t.palette.fg_muted, false).elide(),
            Row::new(children![
                small_btn("󰒮", 44.0, t, |_| Media::previous()),
                small_btn(if playing { "" } else { "" }, 44.0, t, |_| Media::play_pause()),
                small_btn("󰒭", 44.0, t, |_| Media::next()),
            ]).gap(8.0).align(Center),
        ]).width(Parent).gap(6.0)
    )
}

fn battery_section(t: &Theme) -> Rectangle {
    let b = Battery::read();
    if !b.present() {
        return Rectangle::new().width(0.0).height(0.0).fill(Color::TRANSPARENT);
    }
    let p = b.percent();
    let status = if b.charging() { "Charging" } else if b.full() { "Full" } else { "Battery" };
    section(t, "Battery", t.palette.green, 96.0,
        Column::new(children![
            txt(format!("{status}  {p}%"), t.style.font_size - 1.0, t.palette.fg, t.style.bold),
            slider(t, 320.0, p, t.palette.green, |_| {}),
        ]).width(Parent).gap(8.0)
    )
}

fn power_section(t: &Theme) -> Rectangle {
    section(t, "Power", t.palette.red, 86.0,
        Row::new(children![
            small_btn("Reboot", 92.0, t, |_| amane::spawn("systemctl reboot")),
            small_btn("Shutdown", 92.0, t, |_| amane::spawn("systemctl poweroff")),
            small_btn("Lock", 92.0, t, |_| Lock::start()),
        ]).gap(8.0).align(Center)
    )
}

fn small_btn(label: &str, width: f32, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    Rectangle::new().width(width).height(30.0).radius(t.style.button_radius.max(6.0))
        .fill(t.palette.bg1).border(1.0, t.palette.bg3)
        .padding(Padding { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .align_child(Center, Center).cursor(Pointer).on_click(on_click)
        .child(txt(label, t.style.font_size - 2.0, t.palette.fg, t.style.bold))
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
