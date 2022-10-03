use srun::config::Config;
use srun::login::{check_login_status, login, switch_user};
use std::thread::sleep;
use thirtyfour::prelude::WebDriverResult;
use thirtyfour::WebDriver;
#[tokio::main]
async fn main() {
    let mut error_happend: u64 = 0;
    let config = Config::load().await;
    loop {
        let driver = config.init_browser().await;
        match driver {
            Ok(driver) => {
                if error_happend <= config.max_failed() {
                    match daemon(&driver, &config).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("发生错误：\n{}\n准备重启浏览器", err.to_string());
                            error_happend += 1;
                            driver.quit().await.expect("无法正常结束浏览器进程");
                        }
                    }
                } else {
                    panic!("错误次数超过预期，程序退出，请参考日志。")
                }
            }
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}

async fn daemon(driver: &WebDriver, config: &Config) -> WebDriverResult<()> {
    let (username, password) = config.user_info();
    let waiting = config.waiting();
    let login_status = check_login_status(&driver, waiting).await;
    if let Ok(_login_status) = login_status {
        switch_user(&driver, username, password, waiting).await?;
    } else {
        login(&driver, username, password, waiting).await?;
    }
    loop {
        sleep(config.check_interval());
        login(&driver, username, password, waiting).await?
    }
}
