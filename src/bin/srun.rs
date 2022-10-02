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
        let driver_status = driver.try_wait().expect("Can't get status of webdriver");
        let core_status = core.try_wait().expect("Cat get status of srun-core");
        if let Some(_driver_status) = driver_status {
            if let None = core_status {
                core.kill().expect("Failed to kill srun-core, may need to kill manually.");
            }
            break ();
        } else {
            if let Some(_core_status) = core_status {
                driver.kill().expect("Failed to kill webdriver, may need to kill it manually.");
                break ();
            }
        }
        sleep(Duration::from_millis(10000)).await;
    }
}

fn current_dir_file(filename: &str) -> PathBuf {
    let mut current_dir = PathBuf::from(
        current_exe()
            .expect("Failed to get current dir.")
            .parent()
            .expect("Strange path found"),
    );
    current_dir.push(filename);
    #[cfg(windows)]
    current_dir.set_extension("exe");
    current_dir.iter().collect()
}
