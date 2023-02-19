use std::io::{self, Write};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("localhost:1337")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let mut input = String::new();
    print!("Press Enter to register for the courses... ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let handles = driver.windows().await?;
    for handle in handles {
        driver.switch_to_window(handle).await?;
        let submit_button = driver.find(By::ClassName("submit")).await?;
        submit_button.click().await?;
    }

    Ok(())
}
