use srun::config::Config;
use std::{env::current_exe, path::PathBuf, process::Command, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config = Config::load().await;
    let driver_name = config.driver_name();
    let mut driver = Command::new(driver_name)
        .spawn()
        .expect("Failed to start driver process");
    sleep(Duration::from_millis(1000)).await;
    let core_path = current_dir_file("srun-core");
    println!("{:?}", core_path);
    let mut core = Command::new(core_path)
        .spawn()
        .expect("Failed to start srun login core");
    
    loop {
        if let Ok(Some(_status)) = core.try_wait() {
            driver.kill().unwrap();
        } else if let Ok(None) = core.try_wait() {
            sleep(Duration::from_millis(10000)).await;
        } else {
            core.kill().unwrap();
            driver.kill().unwrap();
        }
    }
}

#[cfg(not(windows))]
fn current_dir_file(filename: &str) -> PathBuf {
    let mut current_dir = PathBuf::from(
        current_exe()
            .expect("Failed to get current dir.")
            .parent()
            .expect("Strange path found"),
    );
    current_dir.push(filename);
    current_dir.iter().collect()
}

#[cfg(windows)]
fn current_dir_file(filename: &str) -> PathBuf {
    let mut current_dir = PathBuf::from(
        current_exe()
            .expect("Failed to get current dir.")
            .parent()
            .expect("Strange path found"),
    );
    current_dir.push(filename);
    current_dir.set_extension("exe");
    current_dir.iter().collect()
}
