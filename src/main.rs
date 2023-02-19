use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:1337")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let handles = driver.windows().await?;
    for handle in handles {
        driver.switch_to_window(handle).await?;
        let title = driver.title().await?;
        println!("Title: {}", title);
    }

    Ok(())
}
