use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:1337")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.new_window().await?;
    let handles = driver.windows().await?;
    driver.switch_to_window(handles[handles.len() - 1].clone()).await?;

    Ok(())
}
