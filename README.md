# Farm Gatherer

A Rust-based web scraping tool that automatically collects farm business information from Google Local search results. Farm Gatherer uses browser automation to extract contact details and addresses, then exports the data to CSV format for easy analysis.

## What It Does

Farm Gatherer automates the tedious process of manually collecting farm business contact information from Google Local search results. It:

1. Performs a Google Local search with your query
2. Clicks through each result to reveal detailed information
3. Extracts the business name, phone number, and address
4. Handles pagination to gather up to 50 results
5. Exports all collected data to a CSV file

## Prerequisites

- **Rust** - Install from [rustup.rs](https://rustup.rs/)
- **ChromeDriver** - Browser automation requires ChromeDriver running on a specific port
  - Download from [ChromeDriver Downloads](https://chromedriver.chromium.org/downloads)
  - Or install via package manager (e.g., `brew install chromedriver` on macOS)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/luciano093/Farm-gatherer.git
cd Farm-gatherer
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### 1. Start ChromeDriver

First, start ChromeDriver on a specific port (e.g., 4444):
```bash
chromedriver --port=4444
```

### 2. Run Farm Gatherer

In a new terminal, run the tool with your search query:

```bash
# Basic usage (headless mode)
cargo run --release -- --search "farms near me" --port 4444

# With visible browser
cargo run --release -- --search "organic farms california" --port 4444 --headless false

# Custom output file
cargo run --release -- --search "dairy farms" --port 4444 --output my_farms.csv
```

### Command Line Arguments

| Argument | Required | Description | Default |
|----------|----------|-------------|---------|
| `--search` | Yes | Search query for Google Local | - |
| `--port` | Yes | Port where ChromeDriver is running | - |
| `--headless` | No | Run browser in headless mode | `true` |
| `--output` | No | Output CSV filename | `data.csv` |

## Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests
---

**Note**: Google's structure may change over time, which could break the scraper. If you encounter issues, please open an issue on GitHub.
