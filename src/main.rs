use thirtyfour::prelude::*;
use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> WebDriverResult<()> {
    let url: &str = "https://www.google.com/";
    let driver_host: &str = "http://localhost:4444";

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;

    let driver = WebDriver::new(driver_host, caps).await?;

    driver.get(url).await?;

    let title = driver.title().await?;
    println!("Title: {}", title);

    driver.quit().await?;

    Ok(())
}