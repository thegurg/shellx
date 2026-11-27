use std::cell::RefCell;
use std::time::Duration;

use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

pub struct RailState {
    visible: bool,
    hovered: bool,
}

impl Service for RailState {
    fn new() -> Self { Self { visible: false, hovered: false } }
    fn interval() -> Duration { Duration::from_secs(3600) }
}

pub fn toggle() {
    let mut s = RailState::write();
    s.visible = !s.visible;
}
pub fn show() { RailState::write().visible = true; }
pub fn hide() { RailState::write().visible = false; }
pub fn is_visible() -> bool { RailState::read().visible }

fn hover_enter() {
    let mut s = RailState::write();
    s.hovered = true;
    s.visible = true;
}

fn hover_leave() {
    {
        let mut s = RailState::write();
        s.hovered = false;
    }
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(250));
        let mut s = RailState::write();
        if !s.hovered {
            s.visible = false;
        }
    });
}

thread_local! {
    static SLIDE: RefCell<Animation<f32>> = RefCell::new(Animation::new(-52.0));
}

pub fn view() -> LayerWindow {
    let visible = is_visible();
    let t = theme::current();
    let notif_count = Notifications::read().list().len();
    let target = if visible { 8.0 } else { -52.0 };
    let margin_left = SLIDE.with(|a| {
        let mut a = a.borrow_mut();
        a.to(target);
        a.value().round() as i32
    });

    LayerWindow::new()
        .width(56.0)
        .height(340.0)
        .anchor_vertical(Vertical::Middle)
        .anchor_horizontal(Horizontal::Left)
        .margin(Margin { top: 0, right: 0, bottom: 0, left: margin_left })
        .layer(Layer::Overlay)
        .space(Zone::Ignore)
        .keyboard(Keyboard::None)
        .visible(true)
        .child(panel(t, notif_count))
}

fn panel(t: &Theme, notif_count: usize) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border };
    Rectangle::new()
        .width(Parent).height(Parent)
        .radius(t.style.bar_radius.max(12.0))
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(16.0).offset(4.0, 0.0).opacity(0.30))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 10.0, right: 8.0, bottom: 10.0, left: 8.0 })
        .on_hover(|inside| { if inside { hover_enter(); } else { hover_leave(); } })
        .child(
            Column::new(children![
                rail_btn("󰀻", t.palette.green, t, |_| crate::launcher::toggle()),
                rail_btn(if notif_count > 0 { "󰅸" } else { "󰂚" }, if notif_count > 0 { t.palette.yellow } else { t.palette.blue }, t, |_| crate::notifications::toggle_center()),
                rail_btn("󰄨", t.palette.green, t, |_| crate::monitor::toggle()),
                rail_btn("󰸌", t.palette.purple, t, |_| theme::cycle_theme()),
                rail_btn(match t.id { "ember" => "E", "chroma" => "C", "mist" => "M", _ => "B" }, t.palette.aqua, t, |_| theme::cycle_variant()),
                rail_btn("", t.palette.orange, t, |_| amane::spawn("waypaper")),
            ])
            .width(Parent).height(Parent).gap(10.0).align(Center).justify(Start),
        )
}

fn rail_btn(label: &str, fg: &str, t: &Theme, on_click: impl Fn(Button) + 'static) -> Rectangle {
    let border = if t.style.colorful_modules { fg } else { t.style.module_border };
    Rectangle::new()
        .width(40.0).height(40.0)
        .radius(t.style.button_radius.max(8.0))
        .fill(t.palette.bg1)
        .border(t.style.module_border_width.max(1.0), border)
        .align_child(Center, Center)
        .cursor(Pointer)
        .on_hover(|inside| { if inside { hover_enter(); } else { hover_leave(); } })
        .on_click(on_click)
        .child(txt(label, t.style.font_size + 1.0, fg, true))
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
