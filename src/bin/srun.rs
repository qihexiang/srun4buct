use srun::core::Core;
use std::{env::current_exe, path::PathBuf, time::Duration};
use tokio::{process::Command, select, time::sleep};

#[tokio::main]
async fn main() {
    let core = Core::load().await;
    let driver_name = core.driver_name();
    let mut driver = Command::new(driver_name)
        .spawn()
        .expect("Failed to start driver process");
    sleep(Duration::from_millis(1000)).await;
    let core_path = current_dir_file("srun-core");
    let mut core = Command::new(core_path)
        .spawn()
        .expect("Failed to start srun login core");
    select! {
        status = core.wait() => {
            if let Ok(status) = status {
                println!("srun-core was killed. status code: {}", status);
            } else {
                println!("Losing control of srun-core");
            }
            driver.kill().await.expect("Failed to kill driver process.")
        }
        status = driver.wait() => {
            if let Ok(status) = status {
                println!("webdriver was killed. status code: {}", status);
            } else {
                println!("Losing control of webdriver");
            }
            core.kill().await.expect("Failed to kill srun-core process.")
        }
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
    current_dir
}
