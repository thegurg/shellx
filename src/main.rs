mod active_window;
mod bar;
mod clock;
mod config;
mod control_center;
mod launcher;
mod lock_screen;
mod monitor;
mod notifications;
mod rail;
mod settings;
mod theme;

use amane::App;

fn main() {
    let cfg = config::get();
    let mut app = App::new().font("JetBrainsMono Nerd Font");

    if cfg.bar_enabled {
        app = app.window_per_monitor(bar::view);
    }

    // core overlays
    app = app
        .window(launcher::catcher_view)
        .window(launcher::view)
        .window(control_center::catcher_view)
        .window(control_center::view);

    if cfg.notifications_enabled {
        app = app
            .window(notifications::catcher_view)
            .window(notifications::center_view)
            .window(notifications::popup_view);
    }

    if cfg.rail_enabled {
        app = app.window(rail::view);
    }

    if cfg.lock_enabled {
        app = app.lock(lock_screen::view);
    }

    app = app
        .ipc("theme", theme_ipc)
        .ipc("launcher", launcher_ipc)
        .ipc("control-center", control_center_ipc)
        .ipc("rail", rail_ipc)
        .ipc("lock", lock_ipc)
        .ipc("notifications", notifications_ipc)
        .ipc("monitor", monitor_ipc)
        .ipc("config", config_ipc)
        .ipc("settings", settings_ipc);

    app.run();
}

fn theme_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("current");
    match cmd {
        "current" => theme::current().name.to_string(),
        "list" => theme::all().iter().map(|t| format!("{} ({})", t.name, t.id)).collect::<Vec<_>>().join(", "),
        "cycle" => { theme::cycle(); format!("switched to {}", theme::current().name) }
        "cycle-theme" | "theme" => { theme::cycle_theme(); format!("switched to {}", theme::current().name) }
        "cycle-variant" | "variant" => { theme::cycle_variant(); format!("switched to {}", theme::current().name) }
        name => {
            if theme::set_by_id(name) { format!("switched to {}", theme::current().name) }
            else { format!("unknown theme '{name}', use: ember, chroma, mist, bloom, cycle, list, current") }
        }
    }
}

fn launcher_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("toggle");
    match cmd {
        "toggle" => { launcher::toggle(); "ok".into() }
        "open" => { launcher::open(); "ok".into() }
        "close" => { launcher::close(); "ok".into() }
        "status" => if launcher::is_open() { "open".into() } else { "closed".into() },
        _ => "unknown launcher command, use: toggle, open, close, status".into(),
    }
}

fn control_center_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("toggle");
    match cmd {
        "toggle" => { control_center::toggle(); "ok".into() }
        "open" => { control_center::open(); "ok".into() }
        "close" => { control_center::close(); "ok".into() }
        "status" => if control_center::is_open() { "open".into() } else { "closed".into() },
        _ => "unknown control-center command, use: toggle, open, close, status".into(),
    }
}

fn rail_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("toggle");
    match cmd {
        "toggle" => { rail::toggle(); "ok".into() }
        "show" => { rail::show(); "ok".into() }
        "hide" => { rail::hide(); "ok".into() }
        "status" => if rail::is_visible() { "visible".into() } else { "hidden".into() },
        _ => "unknown rail command, use: toggle, show, hide, status".into(),
    }
}

fn lock_ipc(_args: &[String]) -> String {
    amane::Lock::start();
    "ok".into()
}

fn notifications_ipc(args: &[String]) -> String {
    if !config::get().notifications_enabled { return "notifications disabled in config".into(); }
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("toggle");
    match cmd {
        "toggle" => { notifications::toggle_center(); "ok".into() }
        "open" => { notifications::open_center(); "ok".into() }
        "close" => { notifications::close_center(); "ok".into() }
        "clear" => { amane::Notifications::clear(); "ok".into() }
        "status" => if notifications::is_center_open() { "open".into() } else { "closed".into() },
        _ => "unknown notifications command, use: toggle, open, close, clear, status".into(),
    }
}

fn monitor_ipc(args: &[String]) -> String {
    if !config::get().monitor_enabled { return "monitor disabled in config".into(); }
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("toggle");
    match cmd {
        "toggle" => { monitor::toggle(); "ok".into() }
        "open" => { monitor::open(); "ok".into() }
        "close" => { monitor::close(); "ok".into() }
        _ => "unknown monitor command, use: toggle, open, close".into(),
    }
}

fn config_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("get");
    match cmd {
        "reload" => {
            let res = config::reload();
            let c = config::get();
            let _ = theme::set_by_id(&c.theme);
            res
        }
        "get" => {
            let c = config::get();
            format!("theme={} notification_timeout_secs={} bar={} rail={} notifications={} monitor={} lock={}", c.theme, c.notification_timeout_secs, c.bar_enabled, c.rail_enabled, c.notifications_enabled, c.monitor_enabled, c.lock_enabled)
        }
        "settings" => { settings::open(); "ok".into() }
        _ => "unknown config command, use: get, reload, settings".into(),
    }
}

fn settings_ipc(args: &[String]) -> String {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("open");
    match cmd {
        "open" => { settings::open(); "ok".into() }
        "close" => { settings::close(); "ok".into() }
        _ => "unknown settings command, use: open, close".into(),
    }
}
