use std::time::Duration;
use thirtyfour::prelude::*;

async fn navigate_to_tree(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("http://tree.buct.edu.cn").await
}

pub async fn check_login_status(
    driver: &WebDriver,
    waiting: (Duration, Duration),
) -> WebDriverResult<WebElement> {
    navigate_to_tree(driver).await?;
    let (max_waiting, retry_interval) = waiting;
    let logout_button = driver
        .query(By::Id("logout"))
        .wait(max_waiting, retry_interval)
        .first()
        .await;
    logout_button
}

pub async fn login(
    driver: &WebDriver,
    username: &str,
    password: &str,
    waiting: (Duration, Duration),
) -> WebDriverResult<()> {
    let logout_button = check_login_status(driver, waiting).await;
    match logout_button {
        Ok(_btn) => Ok(()),
        Err(_) => {
            let username_input = driver.query(By::Id("username")).first().await?;
            let password_input = driver.query(By::Id("password")).first().await?;
            let login_button = driver.query(By::Id("login-account")).first().await?;
            username_input.send_keys(username).await?;
            password_input.send_keys(password).await?;
            login_button.click().await?;
            check_login_status(driver, waiting).await?;
            Ok(())
        }
    }
}

pub async fn logout(driver: &WebDriver, waiting: (Duration, Duration)) -> WebDriverResult<()> {
    let (max_waiting, retry_interval) = waiting;
    let logout_button = check_login_status(driver, waiting).await;
    if let Ok(logout_button) = logout_button {
        logout_button.click().await?;
        let confirm = driver
            .query(By::ClassName("btn-confirm"))
            .wait(max_waiting, retry_interval)
            .first()
            .await?;
        confirm.click().await?;
        Ok(())
    } else {
        Ok(())
    }
}

pub async fn switch_user(
    driver: &WebDriver,
    username: &str,
    password: &str,
    waiting: (Duration, Duration),
) -> WebDriverResult<()> {
    logout(driver, waiting).await?;
    login(driver, username, password, waiting).await?;
    Ok(())
}
