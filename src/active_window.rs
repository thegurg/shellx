use std::time::Duration;

use amane::Service;

pub struct ActiveWindow {
    title: String,
}

impl Service for ActiveWindow {
    fn new() -> Self {
        Self { title: read_title() }
    }

    fn interval() -> Duration {
        Duration::from_millis(900)
    }

    fn update(&mut self) -> bool {
        let title = read_title();
        let changed = title != self.title;
        self.title = title;
        changed
    }
}

impl ActiveWindow {
    pub fn title(&self) -> &str {
        &self.title
    }
}

fn wm_name() -> &'static str {
    if std::env::var_os("NIRI_SOCKET").is_some() || std::env::var("XDG_CURRENT_DESKTOP").ok().as_deref() == Some("niri") {
        return "Niri";
    }
    if std::env::var_os("MANGOWC_SOCKET").is_some()
        || std::env::var("XDG_CURRENT_DESKTOP").ok().map(|v| v.to_lowercase().contains("mango")).unwrap_or(false)
        || std::env::var("DESKTOP_SESSION").ok().map(|v| v.to_lowercase().contains("mango")).unwrap_or(false)
    {
        return "Mango";
    }
    "Desktop"
}

fn read_title() -> String {
    let wm = wm_name();
    if wm == "Niri" {
        let out = amane::output("niri msg -j focused-window 2>/dev/null");
        let trimmed = out.trim();
        if trimmed.is_empty() || trimmed == "null" {
            return wm.to_string();
        }
        return extract_json_string(trimmed, "title")
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| wm.to_string());
    }
    if wm == "Mango" {
        let out = amane::output("mmsg get focusing-client 2>/dev/null");
        let trimmed = out.trim();
        if trimmed.is_empty() || trimmed == "null" || trimmed.contains("\"error\"") {
            return wm.to_string();
        }
        return extract_json_string(trimmed, "title")
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| wm.to_string());
    }
    wm.to_string()
}

fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\"", key);
    let idx = json.find(&needle)?;
    let after_key = &json[idx + needle.len()..];
    let colon = after_key.find(':')?;
    let rest = after_key[colon + 1..].trim_start();
    if rest.starts_with("null") {
        return None;
    }
    if !rest.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = rest[1..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => {
                if let Some(esc) = chars.next() {
                    match esc {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{0008}'),
                        'f' => out.push('\u{000C}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let hex: String = chars.by_ref().take(4).collect();
                            if let Ok(code) = u16::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(code as u32) {
                                    out.push(ch);
                                }
                            }
                        }
                        other => out.push(other),
                    }
                }
            }
            other => out.push(other),
        }
    }
    Some(out)
}
