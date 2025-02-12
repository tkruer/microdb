use std::{
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

fn current_utc() -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let total_seconds = since_epoch.as_secs();
    let total_minutes = total_seconds / 60;
    let total_hours = total_minutes / 60;

    let seconds = total_seconds % 60;
    let minutes = total_minutes % 60;
    let hours = total_hours % 24;

    let fmt = format!("(UTC): {:02}:{:02}:{:02}", hours, minutes, seconds);
    fmt
}

pub fn prompt() {
    print!("\x1b[0;32m[INFO] - [{}]\x1b[0m microdb > ", current_utc());
    io::stdout().flush().unwrap();
}
