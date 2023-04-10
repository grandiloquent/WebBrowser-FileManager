pub fn seconds_to_duration(v: u64) -> String {
    let d = std::time::Duration::from_secs(v);
    let seconds = d.as_secs() % 60;
    let minutes = (d.as_secs() / 60) % 60;
    let hours = (d.as_secs() / 60) / 60 % 24;
    let days = (d.as_secs() / 60) / 60 / 24;
    if days > 0 {
        format!("{}:{:0>2}:{:0>2}:{:0>2}", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{:0>2}:{:0>2}", minutes, seconds)
    } else {
        format!("00:{:0>2}", seconds)
    }
}
