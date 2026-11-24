use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

const INPUT_ID: &str = "launcher-input";
const LIST_ID: &str = "launcher-list";
const VISIBLE_ROWS: usize = 7;

pub struct LauncherState {
    open: bool,
    query: String,
    selected: usize,
    offset: usize,
}

impl Service for LauncherState {
    fn new() -> Self {
        Self { open: false, query: String::new(), selected: 0, offset: 0 }
    }
    fn interval() -> std::time::Duration {
        std::time::Duration::from_secs(3600)
    }
}

impl LauncherState {}

pub fn toggle() {
    let mut s = LauncherState::write();
    s.open = !s.open;
    if s.open {
        s.query.clear();
        s.selected = 0;
        s.offset = 0;
        TextInput::set_text(INPUT_ID, "");
    }
}

pub fn open() {
    let mut s = LauncherState::write();
    if !s.open {
        s.open = true;
        s.query.clear();
        s.selected = 0;
        s.offset = 0;
        TextInput::set_text(INPUT_ID, "");
    }
}

pub fn close() {
    let mut s = LauncherState::write();
    s.open = false;
}

pub fn is_open() -> bool {
    LauncherState::read().open
}

pub fn catcher_view() -> LayerWindow {
    let open = LauncherState::read().open;
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
        .child(Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT).on_click(|_| close()))
}

fn filtered_apps(query: &str) -> Vec<amane::DesktopApp> {
    let apps = Apps::read();
    let q = query.trim().to_lowercase();
    let mut out = Vec::new();
    for app in apps.list() {
        let name = app.name();
        let desc = app.description().unwrap_or("");
        if !q.is_empty() {
            let hay = format!("{} {}", name.to_lowercase(), desc.to_lowercase());
            if !hay.contains(&q) { continue; }
        }
        out.push(app.clone());
        if out.len() >= 80 { break; }
    }
    out
}

fn move_selection(delta: i32) {
    let st = LauncherState::read();
    let q = st.query.clone();
    let selected = st.selected;
    let offset = st.offset;
    drop(st);
    let len = filtered_apps(&q).len();
    if len == 0 { return; }
    let next = if delta > 0 {
        (selected + 1).min(len - 1)
    } else {
        selected.saturating_sub(1)
    };
    let mut next_offset = offset;
    if next < next_offset { next_offset = next; }
    if next >= next_offset + VISIBLE_ROWS { next_offset = next + 1 - VISIBLE_ROWS; }
    let max_offset = len.saturating_sub(VISIBLE_ROWS);
    if next_offset > max_offset { next_offset = max_offset; }
    let mut st = LauncherState::write();
    st.selected = next;
    st.offset = next_offset;
}

fn launch_selected() {
    if !is_open() { return; }
    let st = LauncherState::read();
    let q = st.query.clone();
    let selected = st.selected;
    drop(st);
    let apps = filtered_apps(&q);
    if let Some(app) = apps.get(selected) {
        app.launch();
        close();
    }
}

fn handle_key(k: Key) {
    match k {
        Key::Escape => close(),
        Key::Down => move_selection(1),
        Key::Up => move_selection(-1),
        Key::Enter => launch_selected(),
        _ => {}
    }
}

pub fn view() -> LayerWindow {
    let st = LauncherState::read();
    let visible = st.open;
    let query = st.query.clone();
    let selected = st.selected;
    let offset = st.offset;
    drop(st);

    if !visible {
        return LayerWindow::new()
            .width(680.0)
            .height(560.0)
            .anchor_vertical(Vertical::Middle)
            .anchor_horizontal(Horizontal::Middle)
            .margin(Margin { top: 0, right: 0, bottom: 0, left: 0 })
            .layer(Layer::Overlay)
            .space(Zone::Ignore)
            .keyboard(Keyboard::None)
            .visible(true)
            .click_through()
            .child(Rectangle::new().width(Parent).height(Parent).fill(Color::TRANSPARENT));
    }

    let t = theme::current();
    LayerWindow::new()
        .width(680.0)
        .height(560.0)
        .anchor_vertical(Vertical::Middle)
        .anchor_horizontal(Horizontal::Middle)
        .margin(Margin { top: 0, right: 0, bottom: 0, left: 0 })
        .layer(Layer::Overlay)
        .space(Zone::Ignore)
        .keyboard(Keyboard::Exclusive)
        .visible(true)
        .on_key(handle_key)
        .child(panel(t, &query, selected, offset))
}

fn panel(t: &Theme, query: &str, selected: usize, offset: usize) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.blue } else { t.style.bar_border };
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .radius(t.style.bar_radius)
        .fill(t.style.bar_bg)
        .border(t.style.module_border_width.max(1.0), border)
        .shadow(Shadow::drop("#000000").blur(28.0).offset(0.0, 12.0).opacity(0.45))
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 18.0, right: 18.0, bottom: 18.0, left: 18.0 })
        .child(
            Column::new(children![
                header(t),
                search_box(t),
                results(t, query, selected, offset),
            ])
            .width(Parent)
            .height(Parent)
            .gap(14.0),
        )
}

fn header(t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(34.0)
        .align_child(Start, Center)
        .child(
            Row::new(children![
                Text::new("󰀻  Launch")
                    .size(t.style.font_size + 4.0)
                    .color(theme::color(t.palette.fg))
                    .weight(Weight::Bold),
                Rectangle::new().width(Parent).height(1.0),
                Text::new("Esc to close")
                    .size(t.style.font_size - 2.0)
                    .color(theme::color(t.palette.fg_muted)),
            ])
            .width(Parent)
            .height(Parent)
            .align(Center),
        )
}

fn search_box(t: &Theme) -> Rectangle {
    let border = if t.style.colorful_modules { t.palette.aqua } else { t.style.module_border };
    Rectangle::new()
        .width(Parent)
        .height(48.0)
        .radius(t.style.module_radius)
        .fill(t.palette.bg0)
        .border(t.style.module_border_width.max(1.0), border)
        .padding(Padding { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
        .align_child(Start, Center)
        .child(
            TextInput::new(INPUT_ID)
                .width(Parent)
                .size(t.style.font_size + 1.0)
                .color(theme::color(t.palette.fg))
                .placeholder("Search apps...")
                .focused()
                .on_change(|text| {
                    let mut s = LauncherState::write();
                    s.query = text.to_string();
                    s.selected = 0;
                    s.offset = 0;
                })
                .on_submit(|_| {
                    launch_selected();
                }),
        )
}

fn results(t: &Theme, query: &str, selected: usize, offset: usize) -> Rectangle {
    let apps = filtered_apps(query);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let total = apps.len();
    let start = offset.min(total.saturating_sub(1));
    let end = (start + VISIBLE_ROWS).min(total);

    for (i, app) in apps.iter().enumerate().skip(start).take(end - start) {
        let is_sel = i == selected;
        let launched = app.clone();
        let icon = Rectangle::new().width(28.0).height(28.0).radius(6.0);
        let icon = match app.icon_path() {
            Some(path) => icon.fill(Image::contain(path)),
            None => icon.fill(t.palette.bg1).align_child(Center, Center).child(
                Text::new("󰣆").size(t.style.font_size).color(theme::color(t.palette.fg_muted)),
            ),
        };

        let name_text = Text::new(app.name().to_string())
            .size(t.style.font_size)
            .color(theme::color(if is_sel { t.palette.bg_hard } else { t.palette.fg }))
            .weight(if t.style.bold || is_sel { Weight::Bold } else { Weight::Regular });
        let desc_str = app.description().unwrap_or(app.exec());
        let desc_text = Text::new(desc_str.to_string())
            .size(t.style.font_size - 3.0)
            .color(theme::color(if is_sel { t.palette.bg1 } else { t.palette.fg_muted }))
            .elide();

        let (bg, border) = if is_sel {
            (t.palette.blue, t.palette.aqua)
        } else {
            (t.palette.bg0, t.palette.bg2)
        };

        let row = Rectangle::new()
            .width(Parent)
            .height(52.0)
            .radius(t.style.module_radius)
            .fill(bg)
            .border(if is_sel { t.style.module_border_width.max(2.0) } else { 1.0 }, border)
            .padding(Padding { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .align_child(Start, Center)
            .cursor(Pointer)
            .on_click(move |_| {
                launched.launch();
                close();
            })
            .child(
                Row::new(children![
                    icon,
                    Column::new(children![name_text, desc_text]).gap(2.0),
                ])
                .gap(12.0)
                .align(Center),
            );
        rows.push(Box::new(row));
    }

    if rows.is_empty() {
        let q = query.trim();
        rows.push(Box::new(
            Rectangle::new().width(Parent).height(60.0).align_child(Center, Center).child(
                Text::new(if q.is_empty() { "Loading apps..." } else { "No apps match" })
                    .size(t.style.font_size)
                    .color(theme::color(t.palette.fg_muted)),
            ),
        ));
    } else if total > VISIBLE_ROWS {
        rows.push(Box::new(
            Rectangle::new().width(Parent).height(24.0).align_child(Center, Center).child(
                Text::new(format!("{}–{} of {}", start + 1, end, total))
                    .size(t.style.font_size - 3.0)
                    .color(theme::color(t.palette.fg_muted)),
            ),
        ));
    }

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .radius(t.style.module_radius)
        .clip()
        .on_scroll(|s| {
            if s.y > 0.0 { move_selection(1); } else if s.y < 0.0 { move_selection(-1); }
        })
        .child(ScrollArea::new(LIST_ID, Column::new(rows).width(Parent).gap(8.0)).width(Parent).height(Parent))
}

