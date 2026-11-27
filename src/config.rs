use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use amane::Service;

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/persius".into());
    PathBuf::from(home).join(".config/amane/config.conf")
}

#[derive(Clone, Debug)]
pub struct Config {
    pub theme: String,
    pub notification_timeout_secs: u64,
    pub bar_enabled: bool,
    pub rail_enabled: bool,
    pub notifications_enabled: bool,
    pub monitor_enabled: bool,
    pub lock_enabled: bool,
    pub osd_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "chroma".into(),
            notification_timeout_secs: 3,
            bar_enabled: true,
            rail_enabled: true,
            notifications_enabled: true,
            monitor_enabled: true,
            lock_enabled: true,
            osd_enabled: true,
        }
    }
}

pub struct ConfigState {
    pub cfg: Config,
}

impl Service for ConfigState {
    fn new() -> Self {
        let cfg = load_from_disk().unwrap_or_default();
        Self { cfg }
    }
    fn interval() -> Duration { Duration::from_secs(3600) }
}

fn load_from_disk() -> Option<Config> {
    let text = fs::read_to_string(config_path()).ok()?;
    let mut cfg = Config::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((k, v)) = line.split_once('=') else { continue };
        let k = k.trim();
        let v = v.trim().trim_matches('"');
        match k {
            "theme" => cfg.theme = v.to_string(),
            "notification_timeout" | "notification_timeout_secs" => cfg.notification_timeout_secs = v.parse().unwrap_or(cfg.notification_timeout_secs),
            "bar" | "bar_enabled" => cfg.bar_enabled = parse_bool(v, cfg.bar_enabled),
            "rail" | "rail_enabled" => cfg.rail_enabled = parse_bool(v, cfg.rail_enabled),
            "notifications" | "notifications_enabled" => cfg.notifications_enabled = parse_bool(v, cfg.notifications_enabled),
            "monitor" | "monitor_enabled" => cfg.monitor_enabled = parse_bool(v, cfg.monitor_enabled),
            "lock" | "lock_enabled" => cfg.lock_enabled = parse_bool(v, cfg.lock_enabled),
            "osd" | "osd_enabled" => cfg.osd_enabled = parse_bool(v, cfg.osd_enabled),
            _ => {}
        }
    }
    Some(cfg)
}

fn parse_bool(v: &str, default: bool) -> bool {
    match v.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => default,
    }
}

pub fn get() -> Config {
    ConfigState::read().cfg.clone()
}

pub fn reload() -> String {
    let cfg = load_from_disk().unwrap_or_default();
    ConfigState::write().cfg = cfg;
    "ok".into()
}

pub fn save(cfg: &Config) -> std::io::Result<()> {
    let p = config_path();
    if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
    let mut out = String::new();
    out.push_str("# Amane shell config\n");
    out.push_str(&format!("theme = \"{}\"\n", cfg.theme));
    out.push_str(&format!("notification_timeout_secs = {}\n", cfg.notification_timeout_secs));
    out.push_str(&format!("bar_enabled = {}\n", cfg.bar_enabled));
    out.push_str(&format!("rail_enabled = {}\n", cfg.rail_enabled));
    out.push_str(&format!("notifications_enabled = {}\n", cfg.notifications_enabled));
    out.push_str(&format!("monitor_enabled = {}\n", cfg.monitor_enabled));
    out.push_str(&format!("lock_enabled = {}\n", cfg.lock_enabled));
    out.push_str(&format!("osd_enabled = {}\n", cfg.osd_enabled));
    fs::write(p, out)
}

pub fn set_theme(theme: &str) {
    let mut s = ConfigState::write();
    s.cfg.theme = theme.to_string();
    let _ = save(&s.cfg);
}
