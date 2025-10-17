thirtyfour crate allows Rust to navigate web browsers using the WebDriver protocol. It is used in conjunction with the Tokio package to allow multi-threaded async execution.

# What other crates can you use
As of this writing, the most popular Rust libraries for scraping/automation are:

* thirtyfour - Modern and actively maintained. Uses the W3C WebDriver API, so it can drive multiple browsers (Chrome/Chromium, Firefox, Edge, etc.).
* chromiumoxide - A Chromium/CDP client (uses the Chrome DevTools Protocol). Works only with Chromium-based browsers and provides lower-level control than WebDriver clients.
* fantoccini - Maintained and lightweight. Uses the generic WebDriver API but is lower-level than thirtyfour, resulting in less abstraction and a smaller footprint.
  
I chose thirtyfour for this project because it supports multiple browsers. chromiumoxide is Chromium-only, so it didn’t meet the multi-browser requirement. fantoccini is lower-level and often requires more boilerplate than thirtyfour. Furthermore, the WebDriver syntax does not change for whichever browser you choose. Allows easily swappable browers or launch multiple instances in different browsers. If privacy is important, you could say use Firefox instead of Chrome. Any new browers in the future will just require the updated WebDriver binary.

# How does it work
1. Rust application - Write your scraping code using the functions and features of your selected crate (e.g., thirtyfour).
2. WebDriver - Your Rust application sends WebDriver HTTP requests to a WebDriver server (e.g. geckodriver, chromedriver). The driver is a downloadable binary that listens on a port (for example 127.0.0.1:4444) and forwards WebDriver commands to the browser. This means you must run the driver alongside your Rust application when using a WebDriver client like thirtyfour. (If you use a CDP-based client such as chromiumoxide, you can talk directly to Chromium without a separate chromedriver.)
3. Browser - The browser must be installed on the system running the Rust app. The WebDriver launches a new browser instance with specified parameters (window size, headless/headed mode, etc.), and the driver instructs that browser to perform actions.

This will result in your Rust code controlling the browser. This can be used for scraping websites or even website automation.

# Tokio
Tokio provides the async runtime that powers concurrency and (with a multi-threaded runtime) parallel execution. The thirtyfour crate requires an async runtime such as Tokio.

# Prerequisites
* Rust
* Rust crates
  * thirtyfour
  * Tokio
* WebDriver server (e.g., geckodriver for Firefox, chromedriver for Chrome) when using WebDriver clients.
* The web browser installed on the system
* Update the code for `caps`, which stands for `capabilities`. It is currently set to `firefox` but you can simply update (chrome, edge, safari)
* Remove the `headless` line of code if you wish to see the browser automation. 
