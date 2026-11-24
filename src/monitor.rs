use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use amane::*;

use crate::theme::{self, GRAIN_SHADER, Theme};

static OPEN: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Default)]
pub struct FsStat {
    pub mount: String,
    pub used_pct: u8,
    pub used_gib: f32,
    pub total_gib: f32,
}

pub struct FsStats {
    pub items: Vec<FsStat>,
}

impl Service for FsStats {
    fn new() -> Self { let mut s = Self { items: Vec::new() }; s.refresh(); s }
    fn interval() -> Duration { Duration::from_secs(5) }
    fn update(&mut self) -> bool { self.refresh(); true }
}

impl FsStats {
    fn refresh(&mut self) {
        let out = amane::output("df -P -BG --exclude-type=tmpfs --exclude-type=devtmpfs --exclude-type=squashfs --exclude-type=efivarfs --exclude-type=overlay 2>/dev/null");
        let mut items = Vec::new();
        for line in out.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 6 { continue; }
            let total_g: f32 = parts[1].trim_end_matches('G').parse().unwrap_or(0.0);
            let used_g: f32 = parts[2].trim_end_matches('G').parse().unwrap_or(0.0);
            let pct: u8 = parts[4].trim_end_matches('%').parse().unwrap_or(0);
            let mount = parts[5].to_string();
            if total_g <= 0.0 { continue; }
            items.push(FsStat { mount, used_pct: pct, used_gib: used_g, total_gib: total_g });
        }
        items.sort_by(|a,b| a.mount.cmp(&b.mount));
        items.dedup_by(|a,b| a.mount == b.mount);
        self.items = items;
    }
}

pub fn open() {
    OPEN.store(true, Ordering::Relaxed);
    open_window("monitor", view);
}

pub fn close() {
    OPEN.store(false, Ordering::Relaxed);
    close_window("monitor");
}

pub fn toggle() {
    if OPEN.load(Ordering::Relaxed) { close(); } else { open(); }
}

pub fn view() -> Window {
    let t = theme::current();
    Window::new()
        .title("Amane Monitor")
        .size(520.0, 560.0)
        .resizable(true)
        .on_key(|k| { if k == Key::Escape { close(); } })
        .child(panel(t))
}

fn panel(t: &Theme) -> Rectangle {
    let cpu = Cpu::read().percent();
    let mem = Memory::read();
    let mem_pct = mem.percent();
    let mem_used = mem.used_kib() as f32 / 1024.0 / 1024.0;
    let mem_total = mem.total_kib() as f32 / 1024.0 / 1024.0;
    drop(mem);
    let fs = FsStats::read();
    let fs_items = fs.items.clone();
    drop(fs);

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    rows.push(Box::new(header(t)));
    rows.push(Box::new(metric_card(t, "  CPU", cpu, format!("{}%", cpu), t.palette.blue)));
    rows.push(Box::new(metric_card(t, "󰍛  RAM", mem_pct, format!("{:.1} / {:.1} GiB", mem_used, mem_total), t.palette.purple)));
    rows.push(Box::new(section_title(t, "󰋊  Filesystems")));
    if fs_items.is_empty() {
        rows.push(Box::new(txt("No filesystem data", t.style.font_size - 1.0, t.palette.fg_muted, false)));
    } else {
        for fs in fs_items.iter().take(8) {
            let label = format!("{:.0}G / {:.0}G", fs.used_gib, fs.total_gib);
            rows.push(Box::new(metric_card(t, &format!("  {}", fs.mount), fs.used_pct, label, fs_color(t, fs.used_pct))));
        }
    }

    Rectangle::new()
        .width(Parent).height(Parent)
        .fill(t.palette.bg_hard)
        .shader(GRAIN_SHADER)
        .shader_values(vec![[t.style.grain_intensity, t.style.grain_scale, 0.0, 0.0]])
        .padding(Padding { top: 20.0, right: 20.0, bottom: 20.0, left: 20.0 })
        .child(ScrollArea::new("monitor-scroll", Column::new(rows).width(Parent).gap(12.0)).width(Parent).height(Parent))
}

fn header(t: &Theme) -> Rectangle {
    Rectangle::new()
        .width(Parent).height(40.0)
        .fill(Color::TRANSPARENT)
        .child(
            Row::new(children![
                txt("󰄨  System Monitor", t.style.font_size + 5.0, t.palette.green, true),
                Rectangle::new().width(12.0).height(1.0).fill(Color::TRANSPARENT),
                small_btn("Close", 64.0, t, |_| close()),
            ]).width(Parent).height(Parent).gap(8.0).align(Center)
        )
}

fn section_title(t: &Theme, label: &str) -> Text {
    txt(label, t.style.font_size + 1.0, t.palette.orange, true)
}

fn metric_card(t: &Theme, label: &str, pct: u8, detail: String, color: &str) -> Rectangle {
    let pct = pct.min(100);
    Rectangle::new()
        .width(Parent).height(72.0)
        .radius(t.style.module_radius.max(10.0))
        .fill(t.palette.bg0)
        .border(t.style.module_border_width.max(1.0), t.style.module_border)
        .padding(Padding { top: 12.0, right: 12.0, bottom: 12.0, left: 12.0 })
        .child(
            Column::new(children![
                Row::new(children![
                    txt(label, t.style.font_size, t.palette.fg, true),
                    Rectangle::new().width(8.0).height(1.0).fill(Color::TRANSPARENT),
                    txt(detail, t.style.font_size - 1.0, t.palette.fg_muted, false),
                ]).width(Parent).gap(6.0).align(Center),
                progress_bar(t, pct, color),
            ]).width(Parent).gap(8.0)
        )
}

fn progress_bar(t: &Theme, pct: u8, color: &str) -> Rectangle {
    let w = (pct as f32) / 100.0;
    let (ww, _) = amane::window_size();
    let bar_w = if ww > 0.0 { ww - 64.0 } else { 440.0 };
    Rectangle::new()
        .width(Parent).height(12.0)
        .radius(6.0)
        .fill(t.palette.bg2)
        .clip()
        .child(
            Rectangle::new()
                .width((bar_w * w).max(2.0)).height(Parent)
                .radius(6.0)
                .fill(color)
        )
}

fn fs_color(t: &Theme, pct: u8) -> &str {
    if pct >= 90 { t.palette.red }
    else if pct >= 75 { t.palette.yellow }
    else { t.palette.aqua }
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
