use amane::*;

use crate::clock::Clock;
use crate::theme::{self, GRAIN_SHADER, Theme};

const PASS_ID: &str = "lock-password";

pub fn view(_monitor: &Monitor) -> LayerWindow {
    let t = theme::current();
    let lock = Lock::read();
    let checking = lock.checking();
    let failed = lock.failed();
    drop(lock);
    let clock = Clock::read();
    let time = clock.time().to_string();
    let date = clock.date().to_string();
    drop(clock);

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .anchor_vertical(Vertical::Top)
        .anchor_horizontal(Horizontal::Left)
        .layer(Layer::Overlay)
        .space(Zone::Ignore)
        .keyboard(Keyboard::Exclusive)
        .child(
            Rectangle::new()
                .width(Parent).height(Parent)
                .fill(Gradient::linear(180.0, [(0.0, t.palette.bg_hard), (1.0, "#171a1a")]))
                .shader(GRAIN_SHADER)
                .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
                .align_child(Center, Center)
                .child(card(t, &time, &date, checking, failed))
        )
}

fn card(t: &Theme, time: &str, date: &str, checking: bool, failed: bool) -> Rectangle {
    let border = if failed { t.palette.red } else if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border };
    Rectangle::new()
        .width(460.0).height(420.0)
        .radius(t.style.bar_radius.max(16.0))
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(32.0).offset(0.0, 16.0).opacity(0.45))
        .padding(Padding { top: 36.0, right: 36.0, bottom: 36.0, left: 36.0 })
        .align_child(Center, Center)
        .child(
            Column::new(children![
                txt(time, 56.0, t.palette.fg, true),
                txt(date, t.style.font_size + 2.0, t.palette.fg_muted, false),
                Rectangle::new().width(1.0).height(24.0).fill(Color::TRANSPARENT),
                txt(whoami(), t.style.font_size + 4.0, t.palette.yellow, true),
                Rectangle::new().width(1.0).height(8.0).fill(Color::TRANSPARENT),
                password_box(t, checking, failed),
                Rectangle::new().width(1.0).height(8.0).fill(Color::TRANSPARENT),
                txt(
                    if checking { "Authenticating..." } else if failed { "Wrong password, try again" } else { "Enter password to unlock" },
                    t.style.font_size - 1.0,
                    if failed { t.palette.red } else { t.palette.fg_muted },
                    false
                ),
            ])
            .width(Parent).gap(8.0).align(Center),
        )
}

fn password_box(t: &Theme, checking: bool, failed: bool) -> Rectangle {
    let border = if failed { t.palette.red } else { t.palette.blue };
    Rectangle::new()
        .width(Parent).height(48.0)
        .radius(t.style.module_radius.max(8.0))
        .fill(t.palette.bg0)
        .border(t.style.module_border_width.max(1.0), border)
        .padding(Padding { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
        .align_child(Start, Center)
        .child(
            TextInput::new(PASS_ID)
                .width(Parent)
                .size(t.style.font_size + 1.0)
                .color(theme::color(t.palette.fg))
                .placeholder(if checking { "Checking..." } else { "Password" })
                .password()
                .focused()
                .on_submit(|text| {
                    if !text.is_empty() {
                        Lock::unlock(&text);
                    }
                    TextInput::set_text(PASS_ID, "");
                }),
        )
}

fn whoami() -> String {
    amane::output("whoami").trim().to_string()
}

fn txt(s: impl Into<String>, size: f32, color: &str, bold: bool) -> Text {
    let t = Text::new(s.into()).size(size).color(theme::color(color));
    if bold { t.weight(Weight::Bold) } else { t }
}
