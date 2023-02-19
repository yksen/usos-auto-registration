## Requirements
- Google Chrome
- [ChromeDriver](https://chromedriver.chromium.org/downloads)

## Installation
Clone the repository and build the package with
```bash
cargo build --release
```

## Usage
### Windows
Run the **ChromeDriver** executable.

Launch **Google Chrome** with the debugging port `1337` specified

```bash
.\chrome.exe --remote-debugging-port=1337
```
Open tabs with the class groups you want to join already selected.

Run the script with `cargo run` and when the time comes press enter to register for all the courses.
