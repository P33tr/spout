use notify_rust::{Notification, Hint};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
    .summary("minimal notification")
    .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .timeout(0)
    .show()?;
    Ok(())
}
