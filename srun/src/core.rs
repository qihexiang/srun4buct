use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

use crate::actions::{check_login_status, login, switch_user};

fn default_browser() -> String {
    String::from("firefox")
}

fn default_addr() -> String {
    String::from("http://localhost:4444")
}

fn default_interval() -> u64 {
    300_000
}

fn default_headless() -> bool {
    true
}

fn default_waiting() -> (u64, u64) {
    (5000, 1000)
}

fn default_max_failed() -> u64 {
    32
}

#[derive(Serialize, Deserialize)]
pub struct Core {
    username: String,
    password: String,
    #[serde(default = "default_browser")]
    browser: String,
    #[serde(default = "default_addr")]
    addr: String,
    #[serde(default = "default_interval")]
    interval: u64,
    #[serde(default = "default_headless")]
    headless: bool,
    #[serde(default = "default_waiting")]
    waiting: (u64, u64),
    #[serde(default = "default_max_failed")]
    max_failed: u64,
}

impl Core {
    pub async fn load() -> Core {
        let basedirs = BaseDirs::new().expect("无法载入环境目录信息");
        let mut path = PathBuf::from(basedirs.home_dir());
        path.push(".srun.rs.json");
        let path: PathBuf = path.iter().collect();
        let config_file: String = read_to_string(path)
            .expect(format!("无法读取目标文件，请在用户目录下放置.srun.rs.json").as_str());
        let config: Core = serde_json::from_str(&config_file).expect("无法解析文件，需要以下项目：\n字符串：username, password, browser\n数字：interval\n布尔值：headless\n整数二元元组：waiting");
        config
    }

    pub fn driver_name(&self) -> &str {
        match self.browser.to_lowercase().as_str() {
            "firefox" => "geckodriver",
            "chrome" => "chromedriver",
            "edge" => "edgedriver",
            _ => panic!("Unsupported browser. Only support firefox, chrome and edge now."),
        }
    }

    pub async fn init_daemon(&self) -> WebDriverResult<()> {
        let mut error_happend: u64 = 0;
        loop {
            let driver = self.init_browser().await;
            match driver {
                Ok(driver) => {
                    if error_happend <= self.max_failed {
                        match self.daemon_loop(&driver).await {
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

    async fn daemon_loop(&self, driver: &WebDriver) -> WebDriverResult<()> {
        let login_status = check_login_status(&driver, self.waiting()).await;
        if let Ok(_login_status) = login_status {
            switch_user(&driver, &self.username, &self.password, self.waiting()).await?;
        } else {
            login(&driver, &self.username, &self.password, self.waiting()).await?;
        }
        loop {
            sleep(self.check_interval()).await;
            login(&driver, &self.username, &self.password, self.waiting()).await?
        }
    }

    async fn init_browser(&self) -> WebDriverResult<WebDriver> {
        match self.browser.to_lowercase().as_str() {
            "firefox" => {
                let mut caps = DesiredCapabilities::firefox();
                if self.headless {
                    caps.set_headless()?;
                }
                let driver = WebDriver::new(&self.addr, caps).await?;
                Ok(driver)
            }
            "chrome" => {
                let mut caps = DesiredCapabilities::chrome();
                if self.headless {
                    caps.set_headless()?;
                }
                let driver = WebDriver::new(&self.addr, caps).await?;
                Ok(driver)
            }
            "edge" => {
                let caps = DesiredCapabilities::edge();
                let driver = WebDriver::new(&self.addr, caps).await?;
                Ok(driver)
            }
            _ => panic!("Unsupported browser. Only support firefox, chrome and edge now."),
        }
    }

    pub fn user_info<'a>(&self) -> (&str, &str) {
        (&self.username, &self.password)
    }

    pub fn check_interval(&self) -> Duration {
        Duration::from_millis(self.interval)
    }

    pub fn waiting(&self) -> (Duration, Duration) {
        let (timeout, interval) = self.waiting;
        (
            Duration::from_millis(timeout),
            Duration::from_millis(interval),
        )
    }

    pub fn max_failed(&self) -> u64 {
        self.max_failed
    }
}
