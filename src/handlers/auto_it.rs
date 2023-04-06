use rocket::http::Status;
use std::process::Command;
use sysinfo::{SystemExt, ProcessExt};
fn terminate_auto_it_process() -> bool {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );
    let mut result = false;
    for process in sys.processes().values() {
        if process.name() == "AutoIt3.exe" {
            process.kill();
            result = true;
        }
    }
    return result;
}
fn run_auto_it(path: &str) {
    Command::new(r#"C:\Program Files (x86)\AutoIt3\autoit3.exe"#)
        .arg(path)
        .spawn()
        .expect("ls command failed to start");
}
#[get("/autoit?<path>")]
pub async fn auto_it(path: String) -> Result<(), Status> {
    if !terminate_auto_it_process() {
        run_auto_it(path.as_str())
    }
    Ok(())
}