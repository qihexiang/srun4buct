use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Duration;
use thirtyfour::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    username: String,
    password: String,
    browser: String,
    driver: String,
    interval: u64,
    headless: bool,
    waiting: (u64, u64),
    max_failed: u64,
}

impl Config {
    pub async fn load() -> Config {
        let basedirs = BaseDirs::new().expect("无法载入环境目录信息");
        let mut path = PathBuf::from(basedirs.home_dir());
        path.push(".srun.rs.json");
        let path: PathBuf = path.iter().collect();
        let config_file: String = read_to_string(path)
            .expect(format!("无法读取目标文件，请在用户目录下放置.srun.rs.json").as_str());
        let config: Config = serde_json::from_str(&config_file).expect("无法解析文件，需要以下项目：\n字符串：username, password, browser\n数字：interval\n布尔值：headless\n整数二元元组：waiting");
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

    pub async fn init_browser(&self) -> WebDriverResult<WebDriver> {
        match self.browser.to_lowercase().as_str() {
            "firefox" => {
                let mut caps = DesiredCapabilities::firefox();
                if self.headless {
                    caps.set_headless()?;
                }
                let driver = WebDriver::new(&self.driver, caps).await?;
                Ok(driver)
            }
            "chrome" => {
                let mut caps = DesiredCapabilities::chrome();
                if self.headless {
                    caps.set_headless()?;
                }
                let driver = WebDriver::new(&self.driver, caps).await?;
                Ok(driver)
            }
            "edge" => {
                let caps = DesiredCapabilities::edge();
                let driver = WebDriver::new(&self.driver, caps).await?;
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
