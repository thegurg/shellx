use std::time::Duration;

use amane::Service;

pub struct Clock {
    time: String,
    date: String,
}

impl Service for Clock {
    fn new() -> Self {
        Self {
            time: read_time(),
            date: read_date(),
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }

    fn update(&mut self) -> bool {
        let time = read_time();
        let date = read_date();
        let changed = time != self.time || date != self.date;
        self.time = time;
        self.date = date;
        changed
    }
}

impl Clock {
    pub fn time(&self) -> &str {
        &self.time
    }

    pub fn date(&self) -> &str {
        &self.date
    }

}

fn read_time() -> String {
    amane::output("date '+%I:%M %p'").trim().to_string()
}

fn read_date() -> String {
    amane::output("date '+%d %b %a'").trim().to_string()
}
